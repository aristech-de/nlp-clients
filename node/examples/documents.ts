import 'dotenv/config'

import { readFile } from 'node:fs/promises'
import { basename, extname } from 'node:path'

import { Documents, NlpClient } from '@aristech-org/nlp-client'

const { ContentType, processingStatus_StatusToJSON } = Documents

// Maps file extensions to the matching document content type
const contentTypes: Record<string, Documents.ContentType> = {
  '.pdf': ContentType.APPLICATION_PDF,
  '.md': ContentType.APPLICATION_MD,
  '.txt': ContentType.APPLICATION_TXT,
  '.xlsx': ContentType.APPLICATION_XLSX,
  '.png': ContentType.IMAGE_PNG,
  '.jpg': ContentType.IMAGE_JPEG,
  '.jpeg': ContentType.IMAGE_JPEG,
  '.webp': ContentType.IMAGE_WEBP,
}

const projectId = process.env.PROJECT_ID
if (!projectId) {
  throw new Error('PROJECT_ID must be set')
}

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new NlpClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})

// If a file is given as the first argument, add it to the project
const file = process.argv[2]
if (file) {
  const contentType = contentTypes[extname(file).toLowerCase()]
  if (contentType === undefined) {
    throw new Error(`Unsupported file type: ${file}`)
  }
  const added = await client.addDocuments({
    projectId,
    documents: [{
      name: basename(file),
      filename: basename(file),
      contentType,
      data: await readFile(file),
    }],
    // Split the document into intents right away
    autoProcess: true,
  })
  for (const result of added.results) {
    if (result.error) {
      console.error(`Failed to add document: ${result.error}`)
    } else {
      console.log(`Added document ${result.documentId}`)
    }
  }
}

// List the documents of the project with their processing status
const { documents, totalCount } = await client.getDocuments({ projectId })
console.log(`${totalCount} document(s):`)
for (const document of documents) {
  const status = document.processingStatus ? processingStatus_StatusToJSON(document.processingStatus.status) : 'UNKNOWN'
  console.log(`${document.id} ${document.name} [${status}] ${document.intentIds.length} intents`)
}
