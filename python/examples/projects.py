from aristech_nlp_client import NlpClient
from utils import host, auth_token, auth_secret, root_cert, ssl

client = NlpClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
projects = client.list_projects()
for project in projects:
    print(project)
