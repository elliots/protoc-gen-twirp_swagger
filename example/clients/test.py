from __future__ import print_function
import time
import swagger_client
from swagger_client.rest import ApiException
from pprint import pprint
# create an instance of the API class
cfg = swagger_client.Configuration()
cfg.host="http://localhost:8080"
client = swagger_client.ApiClient(cfg)
api_instance = swagger_client.HaberdasherApi(client)
body = swagger_client.ExampleSize() # ExampleSize | 
body.inches = 20

try:
    # MakeHat produces a hat of mysterious, randomly-selected color!
    api_response = api_instance.make_hat(body)
    pprint(api_response)
except ApiException as e:
    print("Exception when calling HaberdasherApi->make_hat: %s\n" % e)