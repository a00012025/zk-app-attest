import urequests
import json

from utilities import load_env, connect_wifi

# Your API endpoint (HTTPS URL)


def send_post_request(json_data, url):
    headers = {
        "Content-Type": "application/json"
    }
    try:
        response = urequests.post(url, data=json_data, headers=headers)
        # Check the response status
        if response.status_code == 200:
            print("Data sent successfully!")
        else:
            print("Failed to send data, status code:", response.status_code)

        # Print the response from the server
        print("Response:", response.text)
        return response.json()

        # Close the response
        response.close()
    except Exception as e:
        print("Error sending data:", e)
