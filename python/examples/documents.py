import sys
from pathlib import Path

from aristech_nlp_client import (
  NlpClient,
  AddDocumentsRequest,
  ContentType,
  Document,
  GetDocumentsRequest,
  ProcessingStatus,
)
from utils import host, auth_token, auth_secret, root_cert, ssl, project_id

# Maps file extensions to the matching document content type
CONTENT_TYPES = {
  ".pdf": ContentType.APPLICATION_PDF,
  ".md": ContentType.APPLICATION_MD,
  ".txt": ContentType.APPLICATION_TXT,
  ".xlsx": ContentType.APPLICATION_XLSX,
  ".png": ContentType.IMAGE_PNG,
  ".jpg": ContentType.IMAGE_JPEG,
  ".jpeg": ContentType.IMAGE_JPEG,
  ".webp": ContentType.IMAGE_WEBP,
}

if not project_id:
  sys.exit("PROJECT_ID must be set")

client = NlpClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)

# If a file is given as the first argument, add it to the project
if len(sys.argv) > 1:
  path = Path(sys.argv[1])
  content_type = CONTENT_TYPES.get(path.suffix.lower())
  if content_type is None:
    sys.exit(f"Unsupported file type: {path}")
  add_response = client.add_documents(AddDocumentsRequest(
    project_id=project_id,
    documents=[Document(
      name=path.name,
      filename=path.name,
      content_type=content_type,
      data=path.read_bytes(),
    )],
    # Split the document into intents right away
    auto_process=True,
  ))
  for result in add_response.results:
    if result.error:
      print(f"Failed to add document: {result.error}", file=sys.stderr)
    else:
      print(f"Added document {result.document_id}")

# List the documents of the project with their processing status
response = client.get_documents(GetDocumentsRequest(project_id=project_id))
print(f"{response.total_count} document(s):")
for document in response.documents:
  status = ProcessingStatus.Status.Name(document.processing_status.status)
  print(f"{document.id} {document.name} [{status}] {len(document.intent_ids)} intents")

client.close()
