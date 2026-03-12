import * as grpc from '@grpc/grpc-js'

import {
  type DeepPartial,
  type FunctionMessage,
  FunctionRequest,
  NLPServerClient,
  RunFunctionsRequest,
  type RunFunctionsResponse,
  SyncDBsRequest,
  type SyncDBsResponse,
} from './generated/nlp_server.js'
import {
  GetContentRequest,
  type GetContentResponse,
  GetIntentsRequest,
  GetKeywordsRequest,
  type GetKeywordsResponse,
  GetScoreLimitsRequest,
  type GetScoreLimitsResponse,
  Intent,
  RemoveContentRequest,
  type RemoveContentResponse,
  UpdateContentRequest,
  type UpdateContentResponse,
} from './generated/intents.js'
import {
  AddProjectRequest,
  type AddProjectResponse,
  EmbeddingModel,
  GetEmbeddingModelsRequest,
  GetProjectsRequest,
  type Project,
  RemoveProjectRequest,
  type RemoveProjectResponse,
  UpdateProjectRequest,
  type UpdateProjectResponse,
} from './generated/projects.js'

import fs from 'fs'
import {
  AddDocumentsRequest,
  type AddDocumentsResponse,
  GetDocumentsRequest,
  type GetDocumentsResponse,
  GetDocumentStatusRequest,
  type GetDocumentStatusResponse,
  ProcessDocumentsRequest,
  type ProcessDocumentsResponse,
  RemoveDocumentsRequest,
  type RemoveDocumentsResponse,
  UpdateDocumentsRequest,
  type UpdateDocumentsResponse
} from './generated/documents.js'

// Re-export generated types
export * as NlpServer from './generated/nlp_server.js'
export * as Projects from './generated/projects.js'
export * as Intents from './generated/intents.js'
export * as Documents from './generated/documents.js'

export interface ConnectionOptions {
  /**
   * The Aristech NLP-Server uri e.g. nlp.example.com
   */
  host?: string
  /**
   * Whether to use SSL/TLS. Automatically enabled when rootCert is provided
   */
  ssl?: boolean
  /**
   * Allows providing a custom root certificate that might not exist
   * in the root certificate chain
   */
  rootCert?: string
  /**
   * Optionally instead of providing a root cert path via `rootCert` the root cert content can be provided directly
   */
  rootCertContent?: string
  /**
   * Further grpc client options
   */
  grpcClientOptions?: grpc.ClientOptions
  /**
   * Authentication options.
   * **Note:** Can only be used in combination with SSL/TLS.
   */
  auth?: {
    token: string
    secret: string
  }
}

export class NlpClient {
  private cOptions: ConnectionOptions

  constructor(options: ConnectionOptions) {
    this.cOptions = options
  }

  /**
   * List all available functions on the server
   * @param request An optional request object
   * @returns A promise that resolves with an array of function messages
   */
  async listFunctions(
    request?: DeepPartial<FunctionRequest>,
  ): Promise<Array<FunctionMessage>> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = FunctionRequest.create(request)
      const stream = client.getFunctions(req)
      const functions: Array<FunctionMessage> = []
      stream.on('data', (data: FunctionMessage) => {
        functions.push(data)
      })
      stream.on('end', () => {
        res(functions)
      })
      stream.on('error', (err) => {
        rej(err)
      })
    })
  }

  /**
   * Performs a processing request with the given request
   * @param request The request object
   * @returns A promise that resolves with the response object
   */
  async runFunctions(
    request: DeepPartial<RunFunctionsRequest>,
  ): Promise<RunFunctionsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = RunFunctionsRequest.create(request)
      client.runFunctions(req, (err, response) => {
        if (err) {
          rej(err)
        } else {
          res(response)
        }
      })
    })
  }

  /**
   * Performs a processing request with the given request
   * @deprecated Use `runFunctions` instead
   * @param request The request object
   * @returns A promise that resolves with the response object
   */
  async process(
    request: DeepPartial<RunFunctionsRequest>,
  ): Promise<RunFunctionsResponse> {
    return this.runFunctions(request)
  }

  /**
   * Get all projects available on the server
   * @param request An optional request object
   * @returns A promise that resolves with an array of projects
   */
  async listProjects(
    request?: DeepPartial<GetProjectsRequest>,
  ): Promise<Array<Project>> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = GetProjectsRequest.create(request)
      const stream = client.getProjects(req)
      const projects: Array<Project> = []
      stream.on('data', (data: Project) => {
        projects.push(data)
      })
      stream.on('end', () => {
        res(projects)
      })
      stream.on('error', (err) => {
        rej(err)
      })
    })
  }

  /**
   * Add a new project to the server
   * @param project The project to add
   * @returns A promise that resolves with the added project
   */
  async addProject(
    request: DeepPartial<AddProjectRequest>,
  ): Promise<AddProjectResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = AddProjectRequest.create(request)
      client.addProject(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Update a project on the server
   * @param project The project to update
   * @returns A promise that resolves with the updated project
   */
  async updateProject(
    request: DeepPartial<UpdateProjectRequest>,
  ): Promise<UpdateProjectResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = UpdateProjectRequest.create(request)
      client.updateProject(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Remove a project from the server
   * @param projectId The id of the project to remove
   * @returns A promise that resolves when the project was removed
   */
  async removeProject(request: DeepPartial<RemoveProjectRequest>): Promise<RemoveProjectResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = RemoveProjectRequest.create(request)
      client.removeProject(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Get all intents for a given project
   * @param request The request object
   * @returns A promise that resolves with the response objects
   */
  async listIntents(
    request: DeepPartial<GetIntentsRequest>,
  ): Promise<Array<Intent>> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = GetIntentsRequest.create(request)
      const stream = client.getIntents(req)
      const intents: Array<Intent> = []
      stream.on('data', (data: Intent) => {
        intents.push(data)
      })
      stream.on('end', () => {
        res(intents)
      })
      stream.on('error', (err) => {
        rej(err)
      })
    })
  }

  /**
   * This function allows to find out good thresholds by providing prompts that should match
   * an intent and negative prompts that should not match an intent.
   *
   * @param request The request object
   * @returns A promise that resolves with the response object
   */
  async getScoreLimits(
    request: DeepPartial<GetScoreLimitsRequest>,
  ): Promise<GetScoreLimitsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = GetScoreLimitsRequest.create(request)
      client.getScoreLimits(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Get the content of a given id
   * @param request The request object
   * @returns A promise that resolves with the response object
   */
  async getContent(
    request: DeepPartial<GetContentRequest>,
  ): Promise<Array<GetContentResponse>> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = GetContentRequest.create(request)
      const stream = client.getContent(req)
      const responses: Array<GetContentResponse> = []
      stream.on('data', (data: GetContentResponse) => {
        responses.push(data)
      })
      stream.on('end', () => {
        res(responses)
      })
      stream.on('error', (err) => {
        rej(err)
      })
    })
  }

  /**
   * List all available embedding models on the server
   * @param request An optional request object
   * @returns A promise that resolves with an array of embedding models
   */
  async listEmbeddingModels(
    request?: DeepPartial<GetEmbeddingModelsRequest>,
  ): Promise<Array<EmbeddingModel>> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = GetEmbeddingModelsRequest.create(request)
      const stream = client.getEmbeddingModels(req)
      const models: Array<EmbeddingModel> = []
      stream.on('data', (data: EmbeddingModel) => {
        models.push(data)
      })
      stream.on('end', () => {
        res(models)
      })
      stream.on('error', (err) => {
        rej(err)
      })
    })
  }

  /**
   * Triggers a database synchronization on the server.
   * @param request An optional request object
   * @returns A promise that resolves with the synchronization response
   */
  async syncDbs(
    request?: DeepPartial<SyncDBsRequest>,
  ): Promise<SyncDBsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = SyncDBsRequest.create(request)
      client.syncDBs(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Get all keywords for a given project
   * @param request The request object
   * @returns A promise that resolves with the response objects
   */
  async getKeywords(
    request: DeepPartial<GetKeywordsRequest>,
  ): Promise<Array<GetKeywordsResponse>> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = GetKeywordsRequest.create(request)
      const stream = client.getKeywords(req)
      const keywords: Array<GetKeywordsResponse> = []
      stream.on('data', (data: GetKeywordsResponse) => {
        keywords.push(data)
      })
      stream.on('end', () => {
        res(keywords)
      })
      stream.on('error', (err) => {
        rej(err)
      })
    })
  }

  /**
   * Updates content inside the vector database
   * @param request The request object
   * @returns A promise that resolves with the response object
   */
  async updateContent(
    request: DeepPartial<UpdateContentRequest>,
  ): Promise<UpdateContentResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = UpdateContentRequest.create(request)
      client.updateContent(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Removes content from the vector database
   * @param request The request object
   * @returns A promise that resolves when the content was removed
   */
  async removeContent(
    request: DeepPartial<RemoveContentRequest>,
  ): Promise<RemoveContentResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = RemoveContentRequest.create(request)
      client.removeContent(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
  * Adds documents to a given project.
  * @param request The request object
  * @return A promise that resolves with the response object
  */
  async addDocuments(
    request: DeepPartial<AddDocumentsRequest>,
  ): Promise<AddDocumentsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = AddDocumentsRequest.create(request)
      client.addDocuments(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
  * Gets all documents for a given project.
  * @param request The request object
  * @return A promise that resolves with the response object
  */
  async getDocuments(
    request: DeepPartial<GetDocumentsRequest>,
  ): Promise<GetDocumentsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = GetDocumentsRequest.create(request)
      client.getDocuments(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
  * Processes spefified documents in a project. 
  * @param request The request object
  * @return A promise that resolves with the response object
  */
  async processDocuments(
    request: DeepPartial<ProcessDocumentsRequest>,
  ): Promise<ProcessDocumentsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = ProcessDocumentsRequest.create(request)
      client.processDocuments(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
  * Updates documents in a project.
  * @param request The request object
  * @return A promise that resolves with the response object
  */
  async updateDocuments(
    request: DeepPartial<UpdateDocumentsRequest>,
  ): Promise<UpdateDocumentsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = UpdateDocumentsRequest.create(request)
      client.updateDocuments(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
  * Removes documents from a project.
  * @param request The request object
  * @return A promise that resolves with the response object
  */
  async removeDocuments(
    request: DeepPartial<RemoveDocumentsRequest>,
  ): Promise<RemoveDocumentsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = RemoveDocumentsRequest.create(request)
      client.removeDocuments(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
  * Gets the processing status of a document in a project. 
  * @param request The request object
  * @return A promise that resolves with the response object
  */
  async getDocumentStatus(
    request: DeepPartial<GetDocumentStatusRequest>,
  ): Promise<GetDocumentStatusResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = GetDocumentStatusRequest.create(request)
      client.getDocumentStatus(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  private getClient() {
    const {
      rootCert: rootCertPath,
      rootCertContent,
      auth,
      grpcClientOptions,
    } = this.cOptions
    let host = this.cOptions.host || 'localhost:8523'
    let ssl = this.cOptions.ssl === true
    let rootCert: Buffer | null = null
    if (rootCertContent) {
      rootCert = Buffer.from(rootCertContent)
    } else if (rootCertPath) {
      rootCert = fs.readFileSync(rootCertPath)
    }
    const sslExplicit = typeof this.cOptions.ssl === 'boolean' || !!rootCert
    const portRe = /[^:]+:([0-9]+)$/
    if (portRe.test(host)) {
      // In case a port was provided but ssl was not specified
      // ssl is assumed when the port matches 8524
      const [, portStr] = host.match(portRe)!
      const hostPort = parseInt(portStr, 10)
      if (!sslExplicit) {
        if (hostPort === 8524) {
          ssl = true
        } else {
          ssl = false
        }
      }
    } else {
      // In case no port was provided, depending on the ssl settings
      // at the default non ssl port 8523 or ssl port 8524
      if (sslExplicit && ssl) {
        host = `${host}:8524`
      } else {
        host = `${host}:8523`
      }
    }

    let creds = grpc.credentials.createInsecure()
    if (ssl || rootCert) {
      creds = grpc.credentials.createSsl(rootCert)
      if (auth) {
        const callCreds = grpc.credentials.createFromMetadataGenerator(
          (_, cb) => {
            const meta = new grpc.Metadata()
            meta.add('token', auth.token)
            meta.add('secret', auth.secret)
            cb(null, meta)
          },
        )
        creds = grpc.credentials.combineChannelCredentials(creds, callCreds)
      }
    }
    return new NLPServerClient(host, creds, grpcClientOptions)
  }
}
