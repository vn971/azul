    #![allow(dead_code, unused_imports)]
    //! DOM to CSS cascading and styling module
    use crate::dll::*;
    use std::ffi::c_void;
    use crate::dom::Dom;
    use crate::css::Css;


    /// `Node` struct
    pub use crate::dll::AzNode as Node;

    impl std::fmt::Debug for Node { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_node_fmt_debug)(self)) } }
    impl Clone for Node { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_node_deep_copy)(self) } }
    impl Drop for Node { fn drop(&mut self) { (crate::dll::get_azul_dll().az_node_delete)(self); } }


    /// `CascadeInfo` struct
    pub use crate::dll::AzCascadeInfo as CascadeInfo;

    impl std::fmt::Debug for CascadeInfo { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_cascade_info_fmt_debug)(self)) } }
    impl Clone for CascadeInfo { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_cascade_info_deep_copy)(self) } }
    impl Drop for CascadeInfo { fn drop(&mut self) { (crate::dll::get_azul_dll().az_cascade_info_delete)(self); } }


    /// `RectStyle` struct
    pub use crate::dll::AzRectStyle as RectStyle;

    impl std::fmt::Debug for RectStyle { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_rect_style_fmt_debug)(self)) } }
    impl Clone for RectStyle { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_rect_style_deep_copy)(self) } }
    impl Drop for RectStyle { fn drop(&mut self) { (crate::dll::get_azul_dll().az_rect_style_delete)(self); } }


    /// `RectLayout` struct
    pub use crate::dll::AzRectLayout as RectLayout;

    impl std::fmt::Debug for RectLayout { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_rect_layout_fmt_debug)(self)) } }
    impl Clone for RectLayout { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_rect_layout_deep_copy)(self) } }
    impl Drop for RectLayout { fn drop(&mut self) { (crate::dll::get_azul_dll().az_rect_layout_delete)(self); } }


    /// `CascadedCssPropertyWithSource` struct
    pub use crate::dll::AzCascadedCssPropertyWithSource as CascadedCssPropertyWithSource;

    impl std::fmt::Debug for CascadedCssPropertyWithSource { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_cascaded_css_property_with_source_fmt_debug)(self)) } }
    impl Clone for CascadedCssPropertyWithSource { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_cascaded_css_property_with_source_deep_copy)(self) } }
    impl Drop for CascadedCssPropertyWithSource { fn drop(&mut self) { (crate::dll::get_azul_dll().az_cascaded_css_property_with_source_delete)(self); } }


    /// `CssPropertySource` struct
    pub use crate::dll::AzCssPropertySource as CssPropertySource;

    impl std::fmt::Debug for CssPropertySource { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_css_property_source_fmt_debug)(self)) } }
    impl Clone for CssPropertySource { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_css_property_source_deep_copy)(self) } }
    impl Drop for CssPropertySource { fn drop(&mut self) { (crate::dll::get_azul_dll().az_css_property_source_delete)(self); } }


    /// `StyledNodeState` struct
    pub use crate::dll::AzStyledNodeState as StyledNodeState;

    impl std::fmt::Debug for StyledNodeState { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_styled_node_state_fmt_debug)(self)) } }
    impl Clone for StyledNodeState { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_styled_node_state_deep_copy)(self) } }
    impl Drop for StyledNodeState { fn drop(&mut self) { (crate::dll::get_azul_dll().az_styled_node_state_delete)(self); } }


    /// `StyledNode` struct
    pub use crate::dll::AzStyledNode as StyledNode;

    impl std::fmt::Debug for StyledNode { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_styled_node_fmt_debug)(self)) } }
    impl Clone for StyledNode { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_styled_node_deep_copy)(self) } }
    impl Drop for StyledNode { fn drop(&mut self) { (crate::dll::get_azul_dll().az_styled_node_delete)(self); } }


    /// `TagId` struct
    pub use crate::dll::AzTagId as TagId;

    impl std::fmt::Debug for TagId { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_tag_id_fmt_debug)(self)) } }
    impl Clone for TagId { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_tag_id_deep_copy)(self) } }
    impl Drop for TagId { fn drop(&mut self) { (crate::dll::get_azul_dll().az_tag_id_delete)(self); } }


    /// `TagIdToNodeIdMapping` struct
    pub use crate::dll::AzTagIdToNodeIdMapping as TagIdToNodeIdMapping;

    impl std::fmt::Debug for TagIdToNodeIdMapping { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_tag_id_to_node_id_mapping_fmt_debug)(self)) } }
    impl Clone for TagIdToNodeIdMapping { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_tag_id_to_node_id_mapping_deep_copy)(self) } }
    impl Drop for TagIdToNodeIdMapping { fn drop(&mut self) { (crate::dll::get_azul_dll().az_tag_id_to_node_id_mapping_delete)(self); } }


    /// `ParentWithNodeDepth` struct
    pub use crate::dll::AzParentWithNodeDepth as ParentWithNodeDepth;

    impl std::fmt::Debug for ParentWithNodeDepth { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_parent_with_node_depth_fmt_debug)(self)) } }
    impl Clone for ParentWithNodeDepth { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_parent_with_node_depth_deep_copy)(self) } }
    impl Drop for ParentWithNodeDepth { fn drop(&mut self) { (crate::dll::get_azul_dll().az_parent_with_node_depth_delete)(self); } }


    /// `ContentGroup` struct
    pub use crate::dll::AzContentGroup as ContentGroup;

    impl std::fmt::Debug for ContentGroup { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_content_group_fmt_debug)(self)) } }
    impl Clone for ContentGroup { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_content_group_deep_copy)(self) } }
    impl Drop for ContentGroup { fn drop(&mut self) { (crate::dll::get_azul_dll().az_content_group_delete)(self); } }


    /// `StyledDom` struct
    pub use crate::dll::AzStyledDom as StyledDom;

    impl StyledDom {
        /// Styles a `Dom` with the given `Css`, returning the `StyledDom` - complexity `O(count(dom_nodes) * count(css_blocks))`: make sure that the `Dom` and the `Css` are as small as possible, use inline CSS if the performance isn't good enough
        pub fn new(dom: Dom, css: Css) -> Self { (crate::dll::get_azul_dll().az_styled_dom_new)(dom, css) }
        /// Appends an already styled list of DOM nodes to the current `dom.root` - complexity `O(count(dom.dom_nodes))`
        pub fn append(&mut self, dom: StyledDom)  { (crate::dll::get_azul_dll().az_styled_dom_append)(self, dom) }
    }

    impl std::fmt::Debug for StyledDom { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_styled_dom_fmt_debug)(self)) } }
    impl Clone for StyledDom { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_styled_dom_deep_copy)(self) } }
    impl Drop for StyledDom { fn drop(&mut self) { (crate::dll::get_azul_dll().az_styled_dom_delete)(self); } }
