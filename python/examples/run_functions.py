from aristech_nlp_client import NlpClient, Function, RunFunctionsRequest
from utils import host, auth_token, auth_secret, root_cert, ssl

client = NlpClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
response = client.run_functions(RunFunctionsRequest(
    input="hallo wie geht es dir",
    functions=[
        Function(id="spell")
    ]
))
print(response.output)

client.close()