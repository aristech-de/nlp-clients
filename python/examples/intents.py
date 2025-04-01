from aristech_nlp_client import NlpClient, GetIntentsRequest
from utils import host, auth_token, auth_secret, root_cert, ssl, project_id

client = NlpClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
intents = client.list_intents(GetIntentsRequest(
    project_id=project_id
))
for intent in intents:
    print(intent)
