import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:app_device_integrity/app_device_integrity.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Alcohol Detector',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // TRY THIS: Try running your application with "flutter run". You'll see
        // the application has a blue toolbar. Then, without quitting the app,
        // try changing the seedColor in the colorScheme below to Colors.green
        // and then invoke "hot reload" (save your changes or press the "hot
        // reload" button in a Flutter-supported IDE, or press "r" if you used
        // the command line to start the app).
        //
        // Notice that the counter didn't reset back to zero; the application
        // state is not lost during the reload. To reset the state, use hot
        // restart instead.
        //
        // This works for code too, not just values: Most code changes can be
        // tested with just a hot reload.
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Alcohol Detector'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  bool? _isDrunk = true;
  int? _alcoholLevel;
  String? _timestamp = '2024-08-11 11:25:11';
  String? _runId = 'cabe2190492d';

  ServerSocket? _serverSocket;

  @override
  void initState() {
    super.initState();

    final appAttestationPlugin = AppDeviceIntegrity();
    String challenge = '35da2fc3-8616-4e3b-8cef-a526792e50fb:1723314948:231';

    appAttestationPlugin
        .getAttestationServiceSupport(challengeString: challenge)
        .then((token) {
      print('tokenReceived: $token');
    });

    _startHttpServer();
  }

  @override
  void dispose() {
    _serverSocket?.close();
    super.dispose();
  }

  void _startHttpServer() async {
    try {
      // Start the HTTP server and listen on a port
      var server = await HttpServer.bind(InternetAddress.anyIPv4, 8000);
      print('HTTP Server started on port 8000');

      await for (HttpRequest request in server) {
        try {
          if (request.method == 'POST') {
            print(
                'Connection from ${request.connectionInfo?.remoteAddress.address}:${request.connectionInfo?.remotePort}');

            String content = await utf8.decoder.bind(request).join();
            print('Message: $content');

            // parse the json
            final json = jsonDecode(content);
            final voltages = json['voltage_array'] as List<dynamic>;
            setState(() {
              _isDrunk = voltages.any((voltage) => voltage > 2);
              // _alcoholLevel = voltages.reduce((maxValue, voltage) =>
              // maxValue > voltage ? maxValue : voltage);
              _timestamp = json['time'];
              _runId = json['run_id'];
            });

            request.response
              ..statusCode = HttpStatus.ok
              ..write('Data received')
              ..close();
          } else {
            request.response
              ..statusCode = HttpStatus.methodNotAllowed
              ..write('Only POST method is allowed')
              ..close();
          }
        } catch (e) {
          print('Error processing request: $e');
          request.response
            ..statusCode = HttpStatus.internalServerError
            ..write('Error processing request')
            ..close();
        }
      }
    } catch (e) {
      print('Error starting HTTP server: $e');
    }
  }

  void _runAttestation() {
    print('Running attestation for $_alcoholLevel');
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            AnimatedContainer(
              duration: Duration(milliseconds: 500),
              width: 150,
              height: 150,
              decoration: BoxDecoration(
                color: _isDrunk == null
                    ? Colors.grey
                    : (_isDrunk == true ? Colors.red : Colors.green),
                shape: BoxShape.circle,
                boxShadow: [
                  BoxShadow(
                    color: Colors.black26,
                    blurRadius: 10,
                    offset: Offset(0, 5),
                  ),
                ],
              ),
              child: Center(
                child: Icon(
                  _isDrunk == null
                      ? Icons.help_outline
                      : (_isDrunk == true ? Icons.warning : Icons.check),
                  color: Colors.white,
                  size: 80,
                ),
              ),
            ),
            SizedBox(height: 20),
            Text(
              _isDrunk == null
                  ? 'Waiting for data...'
                  : (_isDrunk == true
                      ? 'You are drunk!'
                      : 'You are not drunk!'),
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: _isDrunk == null
                    ? Colors.grey
                    : (_isDrunk == true ? Colors.red : Colors.green),
              ),
            ),
            SizedBox(height: 10),
            Text(
              'Timestamp: ${_timestamp ?? 'N/A'}',
              style: TextStyle(fontSize: 16),
            ),
            Text(
              'Run ID: ${_runId ?? 'N/A'}',
              style: TextStyle(fontSize: 16),
            ),
            SizedBox(height: 12),
            if (_isDrunk != null)
              ElevatedButton(
                onPressed: () {
                  _runAttestation();
                },
                child: Text('Run Attestation'),
              ),
          ],
        ),
      ),
    );
  }
}
