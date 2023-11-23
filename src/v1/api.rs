use crate::v1::assistant::{
    AssistantFileObject, AssistantFileRequest, AssistantObject, AssistantRequest, DeletionStatus,
    ListAssistant, ListAssistantFile,
};
use crate::v1::audio::{
    AudioTranscriptionRequest, AudioTranscriptionResponse, AudioTranslationRequest,
    AudioTranslationResponse,
};
use crate::v1::chat_completion::{FunctionCall, ChatCompletionRequest, ChatCompletionResponse,ChatCompletionMessageForResponse,  ChatCompletionMessage, ChatCompletionChoice};
use crate::v1::completion::{CompletionRequest, CompletionResponse};
use crate::v1::edit::{EditRequest, EditResponse};
use crate::v1::embedding::{EmbeddingRequest, EmbeddingResponse};
use crate::v1::error::APIError;
use crate::v1::file::{
    FileDeleteRequest, FileDeleteResponse, FileListResponse, FileRetrieveContentRequest,
    FileRetrieveContentResponse, FileRetrieveRequest, FileRetrieveResponse, FileUploadRequest,
    FileUploadResponse,
};
use crate::v1::fine_tune::{
    CancelFineTuneRequest, CancelFineTuneResponse, CreateFineTuneRequest, CreateFineTuneResponse,
    DeleteFineTuneModelRequest, DeleteFineTuneModelResponse, ListFineTuneEventsRequest,
    ListFineTuneEventsResponse, ListFineTuneResponse, RetrieveFineTuneRequest,
    RetrieveFineTuneResponse,
};
use crate::v1::image::{
    ImageEditRequest, ImageEditResponse, ImageGenerationRequest, ImageGenerationResponse,
    ImageVariationRequest, ImageVariationResponse,
};
use crate::v1::message::{
    CreateMessageRequest, ListMessage, ListMessageFile, MessageFileObject, MessageObject,
    ModifyMessageRequest,
};
use crate::v1::moderation::{CreateModerationRequest, CreateModerationResponse};
use crate::v1::run::{
    CreateRunRequest, CreateThreadAndRunRequest, ListRun, ListRunStep, ModifyRunRequest, RunObject,
    RunStepObject,
};
use crate::v1::thread::{CreateThreadRequest, ModifyThreadRequest, ThreadObject};

use minreq::Response;

const API_URL_V1: &str = "https://api.openai.com/v1";

pub struct Client {
    pub api_endpoint: String,
    pub api_key: String,
    pub organization: Option<String>,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        Self::new_with_endpoint(endpoint, api_key)
    }

    pub fn new_with_endpoint(api_endpoint: String, api_key: String) -> Self {
        Self {
            api_endpoint,
            api_key,
            organization: None,
        }
    }

    pub fn new_with_organization(api_key: String, organization: String) -> Self {
        let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        Self {
            api_endpoint: endpoint,
            api_key,
            organization: organization.into(),
        }
    }

    pub fn build_request(&self, request: minreq::Request, is_beta: bool) -> minreq::Request {
        let mut request = request
            .with_header("Content-Type", "application/json")
            .with_header("Authorization", format!("Bearer {}", self.api_key));
        if let Some(organization) = &self.organization {
            request = request.with_header("openai-organization", organization);
        }
        if is_beta {
            request = request.with_header("OpenAI-Beta", "assistants=v1");
        }
        request
    }

    pub fn post<T: serde::ser::Serialize>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );
        let request = self.build_request(minreq::post(url), Self::is_beta(path));
        let res = request.with_json(params).unwrap().send();
        match res {
            Ok(res) => {
                if (200..=299).contains(&res.status_code) {
                    Ok(res)
                } else {
                    Err(APIError {
                        message: format!("{}: {}", res.status_code, res.as_str().unwrap()),
                    })
                }
            }
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn get(&self, path: &str) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );
        let request = self.build_request(minreq::get(url), Self::is_beta(path));
        let res = request.send();
        match res {
            Ok(res) => {
                if (200..=299).contains(&res.status_code) {
                    Ok(res)
                } else {
                    Err(APIError {
                        message: format!("{}: {}", res.status_code, res.as_str().unwrap()),
                    })
                }
            }
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn delete(&self, path: &str) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );
        let request = self.build_request(minreq::delete(url), Self::is_beta(path));
        let res = request.send();
        match res {
            Ok(res) => {
                if (200..=299).contains(&res.status_code) {
                    Ok(res)
                } else {
                    Err(APIError {
                        message: format!("{}: {}", res.status_code, res.as_str().unwrap()),
                    })
                }
            }
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn completion(&self, req: CompletionRequest) -> Result<CompletionResponse, APIError> {
        let res = self.post("/completions", &req)?;
        let r = res.json::<CompletionResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn edit(&self, req: EditRequest) -> Result<EditResponse, APIError> {
        let res = self.post("/edits", &req)?;
        let r = res.json::<EditResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, APIError> {
        let res = self.post("/images/generations", &req)?;
        let r = res.json::<ImageGenerationResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn image_edit(&self, req: ImageEditRequest) -> Result<ImageEditResponse, APIError> {
        let res = self.post("/images/edits", &req)?;
        let r = res.json::<ImageEditResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> Result<ImageVariationResponse, APIError> {
        let res = self.post("/images/variations", &req)?;
        let r = res.json::<ImageVariationResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn embedding(&self, req: EmbeddingRequest) -> Result<EmbeddingResponse, APIError> {
        let res = self.post("/embeddings", &req)?;
        let r = res.json::<EmbeddingResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_list(&self) -> Result<FileListResponse, APIError> {
        let res = self.get("/files")?;
        let r = res.json::<FileListResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_upload(&self, req: FileUploadRequest) -> Result<FileUploadResponse, APIError> {
        let res = self.post("/files", &req)?;
        let r = res.json::<FileUploadResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_delete(&self, req: FileDeleteRequest) -> Result<FileDeleteResponse, APIError> {
        let res = self.delete(&format!("{}/{}", "/files", req.file_id))?;
        let r = res.json::<FileDeleteResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_retrieve(
        &self,
        req: FileRetrieveRequest,
    ) -> Result<FileRetrieveResponse, APIError> {
        let res = self.get(&format!("{}/{}", "/files", req.file_id))?;
        let r = res.json::<FileRetrieveResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_retrieve_content(
        &self,
        req: FileRetrieveContentRequest,
    ) -> Result<FileRetrieveContentResponse, APIError> {
        let res = self.get(&format!("{}/{}/content", "/files", req.file_id))?;
        let r = res.json::<FileRetrieveContentResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, APIError> {
        let res = self.post("/chat/completions", &req)?;
        let r = res.json::<ChatCompletionResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn continue_chat(
        &self,
        req: &mut ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, APIError> {
        dbg!(&req);
        let res = self.post("/chat/completions", &req)?;
        match res.json::<ChatCompletionResponse>() {
            Ok(reply) => {
                dbg!(&reply);
                match &reply.choices[0] {
                    ChatCompletionChoice {
                        message,
                        ..
                    } => {
                        if let ChatCompletionMessageForResponse {
                            function_call: Some(FunctionCall {
                                name: Some(name),
                                arguments: Some(
                                    args
                                ),
                            }),
                            ..
                        } = message {
                            if name == "execute_rust" {
                                dbg!(args);
                            }
                        } else {
                            //TODO fix that with non-string intermed, or merge of two structs:
                            let raw = serde_json::to_string(message).unwrap();
                            let new: ChatCompletionMessage = serde_json::from_str(&raw).unwrap();

                            req.messages.push(new);
                        }
 
                    }
                }

                Ok(reply)
            },
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn audio_transcription(
        &self,
        req: AudioTranscriptionRequest,
    ) -> Result<AudioTranscriptionResponse, APIError> {
        let res = self.post("/audio/transcriptions", &req)?;
        let r = res.json::<AudioTranscriptionResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn audio_translation(
        &self,
        req: AudioTranslationRequest,
    ) -> Result<AudioTranslationResponse, APIError> {
        let res = self.post("/audio/translations", &req)?;
        let r = res.json::<AudioTranslationResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_fine_tune(
        &self,
        req: CreateFineTuneRequest,
    ) -> Result<CreateFineTuneResponse, APIError> {
        let res = self.post("/fine-tunes", &req)?;
        let r = res.json::<CreateFineTuneResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_fine_tune(&self) -> Result<ListFineTuneResponse, APIError> {
        let res = self.get("/fine-tunes")?;
        let r = res.json::<ListFineTuneResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_fine_tune(
        &self,
        req: RetrieveFineTuneRequest,
    ) -> Result<RetrieveFineTuneResponse, APIError> {
        let res = self.get(&format!("/fine_tunes/{}", req.fine_tune_id))?;
        let r = res.json::<RetrieveFineTuneResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn cancel_fine_tune(
        &self,
        req: CancelFineTuneRequest,
    ) -> Result<CancelFineTuneResponse, APIError> {
        let res = self.post(&format!("/fine_tunes/{}/cancel", req.fine_tune_id), &req)?;
        let r = res.json::<CancelFineTuneResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_fine_tune_events(
        &self,
        req: ListFineTuneEventsRequest,
    ) -> Result<ListFineTuneEventsResponse, APIError> {
        let res = self.get(&format!("/fine-tunes/{}/events", req.fine_tune_id))?;
        let r = res.json::<ListFineTuneEventsResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn delete_fine_tune(
        &self,
        req: DeleteFineTuneModelRequest,
    ) -> Result<DeleteFineTuneModelResponse, APIError> {
        let res = self.delete(&format!("/models/{}", req.model_id))?;
        let r = res.json::<DeleteFineTuneModelResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_moderation(
        &self,
        req: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, APIError> {
        let res = self.post("/moderations", &req)?;
        let r = res.json::<CreateModerationResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_assistant(&self, req: AssistantRequest) -> Result<AssistantObject, APIError> {
        let res = self.post("/assistants", &req)?;
        let r = res.json::<AssistantObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_assistant(&self, assistant_id: String) -> Result<AssistantObject, APIError> {
        let res = self.get(&format!("/assistants/{}", assistant_id))?;
        let r = res.json::<AssistantObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn modify_assistant(
        &self,
        assistant_id: String,
        req: AssistantRequest,
    ) -> Result<AssistantObject, APIError> {
        let res = self.post(&format!("/assistants/{}", assistant_id), &req)?;
        let r = res.json::<AssistantObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn delete_assistant(&self, assistant_id: String) -> Result<DeletionStatus, APIError> {
        let res = self.delete(&format!("/assistants/{}", assistant_id))?;
        let r = res.json::<DeletionStatus>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_assistant(
        &self,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListAssistant, APIError> {
        let mut url = "/assistants".to_owned();
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListAssistant>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_assistant_file(
        &self,
        assistant_id: String,
        req: AssistantFileRequest,
    ) -> Result<AssistantFileObject, APIError> {
        let res = self.post(&format!("/assistants/{}/files", assistant_id), &req)?;
        let r = res.json::<AssistantFileObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<AssistantFileObject, APIError> {
        let res = self.get(&format!("/assistants/{}/files/{}", assistant_id, file_id))?;
        let r = res.json::<AssistantFileObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn delete_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<DeletionStatus, APIError> {
        let res = self.delete(&format!("/assistants/{}/files/{}", assistant_id, file_id))?;
        let r = res.json::<DeletionStatus>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_assistant_file(
        &self,
        assistant_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListAssistantFile, APIError> {
        let mut url = format!("/assistants/{}/files", assistant_id);
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListAssistantFile>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_thread(&self, req: CreateThreadRequest) -> Result<ThreadObject, APIError> {
        let res = self.post("/threads", &req)?;
        let r = res.json::<ThreadObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_thread(&self, thread_id: String) -> Result<ThreadObject, APIError> {
        let res = self.get(&format!("/threads/{}", thread_id))?;
        let r = res.json::<ThreadObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn modify_thread(
        &self,
        thread_id: String,
        req: ModifyThreadRequest,
    ) -> Result<ThreadObject, APIError> {
        let res = self.post(&format!("/threads/{}", thread_id), &req)?;
        let r = res.json::<ThreadObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn delete_thread(&self, thread_id: String) -> Result<DeletionStatus, APIError> {
        let res = self.delete(&format!("/threads/{}", thread_id))?;
        let r = res.json::<DeletionStatus>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_message(
        &self,
        thread_id: String,
        req: CreateMessageRequest,
    ) -> Result<MessageObject, APIError> {
        let res = self.post(&format!("/threads/{}/messages", thread_id), &req)?;
        let r = res.json::<MessageObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_message(
        &self,
        thread_id: String,
        message_id: String,
    ) -> Result<MessageObject, APIError> {
        let res = self.get(&format!("/threads/{}/messages/{}", thread_id, message_id))?;
        let r = res.json::<MessageObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn modify_message(
        &self,
        thread_id: String,
        message_id: String,
        req: ModifyMessageRequest,
    ) -> Result<MessageObject, APIError> {
        let res = self.post(
            &format!("/threads/{}/messages/{}", thread_id, message_id),
            &req,
        )?;
        let r = res.json::<MessageObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_messages(&self, thread_id: String) -> Result<ListMessage, APIError> {
        let res = self.get(&format!("/threads/{}/messages", thread_id))?;
        let r = res.json::<ListMessage>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_message_file(
        &self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> Result<MessageFileObject, APIError> {
        let res = self.get(&format!(
            "/threads/{}/messages/{}/files/{}",
            thread_id, message_id, file_id
        ))?;
        let r = res.json::<MessageFileObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_message_file(
        &self,
        thread_id: String,
        message_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListMessageFile, APIError> {
        let mut url = format!("/threads/{}/messages/{}/files", thread_id, message_id);
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListMessageFile>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_run(
        &self,
        thread_id: String,
        req: CreateRunRequest,
    ) -> Result<RunObject, APIError> {
        let res = self.post(&format!("/threads/{}/runs", thread_id), &req)?;
        let r = res.json::<RunObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_run(&self, thread_id: String, run_id: String) -> Result<RunObject, APIError> {
        let res = self.get(&format!("/threads/{}/runs/{}", thread_id, run_id))?;
        let r = res.json::<RunObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn modify_run(
        &self,
        thread_id: String,
        run_id: String,
        req: ModifyRunRequest,
    ) -> Result<RunObject, APIError> {
        let res = self.post(&format!("/threads/{}/runs/{}", thread_id, run_id), &req)?;
        let r = res.json::<RunObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_run(
        &self,
        thread_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRun, APIError> {
        let mut url = format!("/threads/{}/runs", thread_id);
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListRun>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn cancel_run(&self, thread_id: String, run_id: String) -> Result<RunObject, APIError> {
        let empty_req = ModifyRunRequest::new();
        let res = self.post(
            &format!("/threads/{}/runs/{}/cancel", thread_id, run_id),
            &empty_req,
        )?;
        let r = res.json::<RunObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_thread_and_run(
        &self,
        req: CreateThreadAndRunRequest,
    ) -> Result<RunObject, APIError> {
        let res = self.post("/threads/runs", &req)?;
        let r = res.json::<RunObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> Result<RunStepObject, APIError> {
        let res = self.get(&format!(
            "/threads/{}/runs/{}/steps/{}",
            thread_id, run_id, step_id
        ))?;
        let r = res.json::<RunStepObject>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_run_step(
        &self,
        thread_id: String,
        run_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRunStep, APIError> {
        let mut url = format!("/threads/{}/runs/{}/steps", thread_id, run_id);
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListRunStep>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    fn new_error(&self, err: minreq::Error) -> APIError {
        APIError {
            message: err.to_string(),
        }
    }

    fn is_beta(path: &str) -> bool {
        path.starts_with("/assistants") || path.starts_with("/threads")
    }

    fn query_params(
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
        mut url: String,
    ) -> String {
        let mut params = vec![];
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(before) = before {
            params.push(format!("before={}", before));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }
        url
    }
}
