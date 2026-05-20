use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::core::router::logic::RouteMetadata;
use crate::core::router::handlers::layout::LayoutProps;
use crate::core::router::io::request::{Request, Connection};
use crate::core::router::io::response::ResponseJar;

#[derive(Clone, Debug)]
pub struct Page {
    inner: Arc<PageInner>,
}

impl std::ops::Deref for Page {
    type Target = PageInner;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub struct PageInner {
    pub(crate) connection: Connection,
    pub(crate) response: RwLock<ResponseJar>,

    pub(crate) is_dynamic: AtomicBool,
    pub(crate) is_modified: AtomicBool,

    pub(crate) metadata: RwLock<RouteMetadata>,
    pub(crate) layout_props: RwLock<LayoutProps>,
}

impl std::ops::Deref for PageInner {
    type Target = Connection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl Page {
    pub(crate) fn new(
        connection: Connection,
        metadata: RouteMetadata,
    ) -> Self {
        Self {
            inner: Arc::new(
                PageInner {
                    connection,
                    response: RwLock::new(ResponseJar::new()),

                    is_dynamic: AtomicBool::new(false),
                    is_modified: AtomicBool::new(false),

                    metadata: RwLock::new(metadata),
                    layout_props: RwLock::new(LayoutProps::default())
                }
            )
        }
    }
}


impl Request<Page> {
    // --- Constructional & Side-Effect Tracking APIs ---

    /// Mark the handler request as dynamic (bypasses static generation)
    #[inline]
    pub(crate) fn mark_dynamic(&self) {
        self.inner.is_dynamic.store(true, Ordering::Relaxed);
    }

    /// Mark that a handler has modified contextual traits (Extensions, Metadata, etc.)
    #[inline]
    pub(crate) fn mark_modified(&self) {
        self.inner.is_modified.store(true, Ordering::Relaxed);
        self.inner.is_dynamic.store(true, Ordering::Relaxed);
    }

    /// Update Layout Props after creating the Request wrapper
    #[inline]
    pub(crate) fn set_layout_props(&self, layout_props: LayoutProps) {
        // FIXED: Explicitly dereference the lock guard to write new values
        *self.inner.layout_props.write() = layout_props;
    }

    /// Acquire safe read access to the Page's Layout Properties
    #[inline]
    pub fn layout_props(&self) -> RwLockReadGuard<'_, LayoutProps> {
        self.mark_dynamic();
        self.inner.layout_props.read()
    }

    /// Acquire safe read access to the Page's Route Configuration Metadata
    #[inline]
    pub fn metadata(&self) -> RwLockReadGuard<'_, RouteMetadata> {
        self.mark_dynamic();
        self.inner.metadata.read()
    }

    #[inline]
    pub fn metadata_mut(&self) -> RwLockWriteGuard<'_, RouteMetadata> {
        // Marks state modifications to bypass static caching layers
        self.mark_modified();
        self.inner.metadata.write()
    }

    #[inline]
    pub fn response(&self) -> RwLockWriteGuard<'_, ResponseJar> {
        self.inner.response.write()
    }
}
