import urequests
import json

from utilities import load_env

# Your API endpoint (HTTPS URL)
env = load_env()
url = env.get('API_URL', 'default_API_URL')

# Data to send
data = {
    "sensor_id": 1,
    "value": 23.5
}

# Convert data to JSON format
headers = {
    "Content-Type": "application/json"
}
json_data = json.dumps(data)

try:
    # Send POST request with JSON data
    response = urequests.post(url, data=json_data, headers=headers)

    # Check the response status
    if response.status_code == 200:
        print("Data sent successfully!")
    else:
        print("Failed to send data, status code:", response.status_code)

    # Print the response from the server
    print("Response:", response.text)

    # Close the response
    response.close()

except Exception as e:
    print("Error sending data:", e)
