from aristech_nlp_client import NlpClient, GetScoreLimitsRequest
from utils import host, auth_token, auth_secret, root_cert, ssl, project_id

client = NlpClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
response = client.score_limits(GetScoreLimitsRequest(
  project_id=project_id,
  test_sentences_upper_limit=[
    "What are the lottery numbers?",
    "Can you tell me the winning numbers?",
  ],
  test_sentences_lower_limit=[
    "What's the weather like?",
    "I want to speak to a human.",
],
))
print(response)
