import 'dart:convert';
import 'dart:io';

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
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Alcohol Detector'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int? _alcoholLevel = null;
  bool? _isDrunk = null;
  String? _timestamp = null;
  String? _runId = null;
  // int? _alcoholLevel = 213;
  // bool? _isDrunk = true;
  // String? _timestamp = '2024-08-11 11:25:11';
  // String? _runId = 'cabe2190492d';

  late final AppDeviceIntegrity appAttestationPlugin;
  ServerSocket? _serverSocket;
  String? _token;

  @override
  void initState() {
    super.initState();

    appAttestationPlugin = AppDeviceIntegrity();
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
              _alcoholLevel = ((voltages.reduce((maxValue, voltage) =>
                          maxValue > voltage ? maxValue : voltage) *
                      100) as double)
                  .round();
              _timestamp = DateTime.now().toString();
              _runId = 'c190311eb92d';
              // _timestamp = json['time'];
              // _runId = json['run_id'];
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

  void _runAttestation() async {
    print('Running attestation for $_alcoholLevel');
    final timestamp =
        (DateTime.now().millisecondsSinceEpoch ~/ 1000).toString();
    final challenge =
        '35da2fc3-8616-4e3b-8cef-a526792e50fb:$timestamp:${_alcoholLevel ?? 0}';
    final token = await appAttestationPlugin.getAttestationServiceSupport(
        challengeString: challenge);
    setState(() {
      _token = token?.replaceAll(RegExp(r'\/'), '/');
    });
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
            if (_token != null)
              Padding(
                padding: const EdgeInsets.all(18.0),
                child: Column(
                  children: [
                    Text(
                      'Token: ${_token!.substring(0, 100)}...${_token!.substring(_token!.length - 100)}',
                      style: TextStyle(fontSize: 16),
                    ),
                    const SizedBox(height: 12),
                    ElevatedButton(
                        onPressed: () async {
                          await Future.delayed(Duration(seconds: 1));
                          // show modal about successfully submitted
                          showDialog(
                            context: context,
                            builder: (context) => AlertDialog(
                              title: Text('Proof Submitted'),
                              content: Column(
                                mainAxisSize: MainAxisSize.min,
                                children: [
                                  Icon(
                                    Icons.check_circle,
                                    color: Colors.green,
                                    size: 64,
                                  ),
                                  SizedBox(height: 16),
                                  Text(
                                    'Your proof has been submitted successfully!',
                                    textAlign: TextAlign.center,
                                  ),
                                ],
                              ),
                            ),
                          );
                        },
                        child: Text('Submit Proof'))
                  ],
                ),
              ),
          ],
        ),
      ),
    );
  }
}
