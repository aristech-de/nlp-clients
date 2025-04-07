import sys

from aristech_nlp_client import NlpClient, GetContentRequest
from utils import host, auth_token, auth_secret, root_cert, ssl, project_id

prompt = sys.argv[1] if len(sys.argv) > 1 else "What are the lottery numbers?"

client = NlpClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
responses = client.get_content(GetContentRequest(
    prompt=prompt,
    threshold=0.5,
    num_results=3,
    project_id=project_id,
))

try:
    print(next(responses))
    for response in responses:
        print(response) 
except StopIteration:
    print("No matching intents above the threshold.")

# To update content:
"""
from aristech_nlp_client import UpdateContentRequest, Intent, IntentInput
updateResponse = client.update_content(UpdateContentRequest(
    intent=[Intent(
        id="123",
        projectId=project_id,
        topic="lottery-numbers",
        inputs=[
            IntentInput(uuid="82a2133e-038e-4c83-aa26-de47ed386c55", input="What are the lottery numbers?"),
            IntentInput(uuid="2cc467ca-dc1a-4d33-92ff-8f73bb941fb9", input="Can you tell me the numbers from saturday?"),
        ],
        output_chat=[
            "The latest lottery numbers are {{lotto_numbers}}"
        ]
    )]
))
"""

# To remove content:
"""
from aristech_nlp_client import RemoveContentRequest
removeResponse = client.remove_content(RemoveContentRequest(
    id=["82a2133e-038e-4c83-aa26-de47ed386c55"],
    projectId=project_id,
))
"""
