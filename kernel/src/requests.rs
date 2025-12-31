use limine::{
  BaseRevision,
  paging::Mode,
  request::{
    FramebufferRequest, HhdmRequest, MemoryMapRequest, PagingModeRequest, RequestsEndMarker,
    RequestsStartMarker,
  },
  response::{HhdmResponse, MemoryMapResponse},
};
use spin::lazy::Lazy;

// #[used] lets the compiler know not to remove these
#[used]
#[unsafe(link_section = ".requests")]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static PAGING_MODE_REQUEST: PagingModeRequest =
  PagingModeRequest::new().with_mode(Mode::FOUR_LEVEL);

#[used]
#[unsafe(link_section = ".requests_start_marker")]
pub static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
pub static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

pub static MEMORY_MAP_RESPONSE: Lazy<&MemoryMapResponse> = Lazy::new(|| {
  MEMORY_MAP_REQUEST
    .get_response()
    .expect("verify() was not called before accessing the memory map response")
});

pub static HHDM_RESPONSE: Lazy<&HhdmResponse> = Lazy::new(|| {
  HHDM_REQUEST
    .get_response()
    .expect("verify() was not called before accessing the HHDM response")
});
