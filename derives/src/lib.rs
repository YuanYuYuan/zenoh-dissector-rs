use proc_macro::TokenStream;
use anyhow::{bail, Result};
use quote::ToTokens;

#[proc_macro_derive(MyProto)]
pub fn derive_proto(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ret = derive_proto_impl(&input)
        .map(|x| x.to_token_stream())
        .expect("Error!!!!!!");
    ret.into()
}

fn derive_proto_impl(input: &syn::DeriveInput) -> Result<syn::ItemImpl> {
    if !matches!(input.data, syn::Data::Struct { .. }) {
        bail!("{:?} only structs can derive Proto", &input);
    }

    let ident = &input.ident;
    Ok(syn::parse_quote! {
        impl dissector::MyProtoTrait for #ident {

        }
    })

    // let ident = &input.ident;
    //
    // let proto_opts = init_options::<ProtocolOptions>(&input.attrs)?;
    // if proto_opts.decode_from.is_empty() {
    //     return make_err(&input.ident, "missing `decode_from` attribute");
    // }
    //
    // let add_dissector = proto_opts.decode_from.iter().map(DecodeFrom::to_tokens);
    //
    // let upper_cased = input.ident.to_wsdf_upper_case();
    // let snake_cased = input.ident.to_wsdf_snake_case();
    //
    // let proto_desc = proto_opts.proto_desc.as_ref().unwrap_or(&upper_cased);
    // let proto_name = proto_opts.proto_name.as_ref().unwrap_or(&upper_cased);
    // let proto_filter = proto_opts.proto_filter.as_ref().unwrap_or(&snake_cased);
    //
    // let proto_desc_cstr: syn::Expr = cstr!(proto_desc);
    // let proto_name_cstr: syn::Expr = cstr!(proto_name);
    // let proto_filter_cstr: syn::Expr = cstr!(proto_filter);
    //
    // Ok(parse_quote! {
    //     impl wsdf::Proto for #ident {
    //         #[allow(clippy::missing_safety_doc)]
    //         unsafe extern "C" fn dissect_main(
    //             tvb: *mut wsdf::epan_sys::tvbuff,
    //             pinfo: *mut wsdf::epan_sys::_packet_info,
    //             tree: *mut wsdf::epan_sys::_proto_node,
    //             data: *mut std::ffi::c_void,
    //         ) -> std::ffi::c_int {
    //             // Clear columns
    //             wsdf::epan_sys::col_set_str(
    //                 (*pinfo).cinfo,
    //                 wsdf::epan_sys::COL_PROTOCOL as std::ffi::c_int,
    //                 #proto_desc_cstr,
    //             );
    //             wsdf::epan_sys::col_clear(
    //                 (*pinfo).cinfo,
    //                 wsdf::epan_sys::COL_INFO as std::ffi::c_int,
    //             );
    //
    //             // Initialize rust-owned TVB
    //             let tvb_len = unsafe {
    //                 wsdf::epan_sys::tvb_reported_length(tvb) as usize
    //             };
    //             let mut tvb_buf = Vec::new();
    //             tvb_buf.resize(tvb_len, 0);
    //             unsafe {
    //                 wsdf::epan_sys::tvb_memcpy(
    //                     tvb,
    //                     tvb_buf.as_mut_ptr() as *mut std::ffi::c_void,
    //                     0,
    //                     tvb_len,
    //                 );
    //             }
    //
    //             __WSDF_HF_INDICES.with(|hf_indices|
    //                 __WSDF_ETT_INDICES.with(|etts|
    //                     __WSDF_DTABLES.with(|dtables| {
    //                         // create the packet-lifespan fields store
    //                         let mut fields = wsdf::FieldsStore::default();
    //
    //                         let hf_indices = hf_indices.borrow();
    //                         let etts = etts.borrow();
    //                         let dtables = dtables.borrow();
    //
    //                         let args = wsdf::DissectorArgs {
    //                             hf_indices: &hf_indices,
    //                             etts: &etts,
    //                             dtables: &dtables,
    //                             tvb,
    //                             pinfo,
    //                             proto_root: tree,
    //                             data: &tvb_buf,
    //                             prefix: #proto_filter,
    //                             prefix_local: #proto_filter,
    //                             offset: 0,
    //                             parent: tree,
    //                             variant: std::option::Option::None,
    //                             list_len: std::option::Option::None,
    //                             ws_enc: std::option::Option::None,
    //                         };
    //
    //                         <#ident as Dissect<'_, ()>>::add_to_tree(&args, &mut fields) as _
    //                     })
    //                 )
    //             )
    //         }
    //
    //         unsafe extern "C" fn register_protoinfo() {
    //             let proto_id = unsafe {
    //                 wsdf::epan_sys::proto_register_protocol(
    //                     #proto_desc_cstr,
    //                     #proto_name_cstr,
    //                     #proto_filter_cstr,
    //                 )
    //             };
    //
    //
    //             __WSDF_HF_INDICES.with(|hf_indices|
    //                 __WSDF_ETT_INDICES.with(|etts|
    //                     __WSDF_DTABLES.with(|dtables| {
    //                         let mut hf = hf_indices.borrow_mut();
    //                         let mut ett = etts.borrow_mut();
    //                         let mut dtable = dtables.borrow_mut();
    //                         let mut ws_indices = wsdf::WsIndices {
    //                             hf: &mut hf,
    //                             ett: &mut ett,
    //                             dtable: &mut dtable,
    //                         };
    //
    //                         ws_indices.hf.insert(#proto_filter, proto_id);
    //
    //                         let args = wsdf::RegisterArgs {
    //                             proto_id,
    //                             name: #proto_name_cstr,
    //                             prefix: #proto_filter,
    //                             blurb: std::ptr::null(),
    //                             ws_type: std::option::Option::None,
    //                             ws_display: std::option::Option::None,
    //                         };
    //
    //                         <#ident as Dissect<'_, ()>>::register(&args, &mut ws_indices);
    //                     })
    //                 )
    //             );
    //         }
    //
    //         unsafe extern "C" fn register_handoff() {
    //             __WSDF_HF_INDICES.with(|hf_indices| {
    //                 let hf_indices = hf_indices.borrow();
    //                 let proto_id = hf_indices.get(#proto_filter).unwrap();
    //                 unsafe {
    //                     let handle = wsdf::epan_sys::create_dissector_handle(
    //                         std::option::Option::Some(<#ident as wsdf::Proto>::dissect_main),
    //                         proto_id,
    //                     );
    //                     #(#add_dissector)*
    //                 }
    //             });
    //         }
    //     }
    // })
}
