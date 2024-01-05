#[inline]
pub unsafe fn WSManCloseCommand<P0>(commandhandle: P0, flags: u32, r#async: *const WSMAN_SHELL_ASYNC)
where
    P0: ::windows_core::IntoParam<WSMAN_COMMAND_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManCloseCommand(commandhandle : WSMAN_COMMAND_HANDLE, flags : u32, r#async : *const WSMAN_SHELL_ASYNC));
    WSManCloseCommand(commandhandle.into_param().abi(), flags, r#async)
}
#[inline]
pub unsafe fn WSManCloseOperation<P0>(operationhandle: P0, flags: u32) -> u32
where
    P0: ::windows_core::IntoParam<WSMAN_OPERATION_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManCloseOperation(operationhandle : WSMAN_OPERATION_HANDLE, flags : u32) -> u32);
    WSManCloseOperation(operationhandle.into_param().abi(), flags)
}
#[inline]
pub unsafe fn WSManCloseSession<P0>(session: P0, flags: u32) -> u32
where
    P0: ::windows_core::IntoParam<WSMAN_SESSION_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManCloseSession(session : WSMAN_SESSION_HANDLE, flags : u32) -> u32);
    WSManCloseSession(session.into_param().abi(), flags)
}
#[inline]
pub unsafe fn WSManCloseShell<P0>(shellhandle: P0, flags: u32, r#async: *const WSMAN_SHELL_ASYNC)
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManCloseShell(shellhandle : WSMAN_SHELL_HANDLE, flags : u32, r#async : *const WSMAN_SHELL_ASYNC));
    WSManCloseShell(shellhandle.into_param().abi(), flags, r#async)
}
#[inline]
pub unsafe fn WSManConnectShell<P0, P1, P2>(session: P0, flags: u32, resourceuri: P1, shellid: P2, options: ::core::option::Option<*const WSMAN_OPTION_SET>, connectxml: ::core::option::Option<*const WSMAN_DATA>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_SHELL_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SESSION_HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManConnectShell(session : WSMAN_SESSION_HANDLE, flags : u32, resourceuri : ::windows_core::PCWSTR, shellid : ::windows_core::PCWSTR, options : *const WSMAN_OPTION_SET, connectxml : *const WSMAN_DATA, r#async : *const WSMAN_SHELL_ASYNC, shell : *mut WSMAN_SHELL_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManConnectShell(session.into_param().abi(), flags, resourceuri.into_param().abi(), shellid.into_param().abi(), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), ::core::mem::transmute(connectxml.unwrap_or(::std::ptr::null())), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[inline]
pub unsafe fn WSManConnectShellCommand<P0, P1>(shell: P0, flags: u32, commandid: P1, options: ::core::option::Option<*const WSMAN_OPTION_SET>, connectxml: ::core::option::Option<*const WSMAN_DATA>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_COMMAND_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManConnectShellCommand(shell : WSMAN_SHELL_HANDLE, flags : u32, commandid : ::windows_core::PCWSTR, options : *const WSMAN_OPTION_SET, connectxml : *const WSMAN_DATA, r#async : *const WSMAN_SHELL_ASYNC, command : *mut WSMAN_COMMAND_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManConnectShellCommand(shell.into_param().abi(), flags, commandid.into_param().abi(), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), ::core::mem::transmute(connectxml.unwrap_or(::std::ptr::null())), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[inline]
pub unsafe fn WSManCreateSession<P0, P1>(apihandle: P0, connection: P1, flags: u32, serverauthenticationcredentials: ::core::option::Option<*const WSMAN_AUTHENTICATION_CREDENTIALS>, proxyinfo: ::core::option::Option<*const WSMAN_PROXY_INFO>, session: *mut WSMAN_SESSION_HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<WSMAN_API_HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManCreateSession(apihandle : WSMAN_API_HANDLE, connection : ::windows_core::PCWSTR, flags : u32, serverauthenticationcredentials : *const WSMAN_AUTHENTICATION_CREDENTIALS, proxyinfo : *const WSMAN_PROXY_INFO, session : *mut WSMAN_SESSION_HANDLE) -> u32);
    WSManCreateSession(apihandle.into_param().abi(), connection.into_param().abi(), flags, ::core::mem::transmute(serverauthenticationcredentials.unwrap_or(::std::ptr::null())), ::core::mem::transmute(proxyinfo.unwrap_or(::std::ptr::null())), session)
}
#[inline]
pub unsafe fn WSManCreateShell<P0, P1>(session: P0, flags: u32, resourceuri: P1, startupinfo: ::core::option::Option<*const WSMAN_SHELL_STARTUP_INFO_V11>, options: ::core::option::Option<*const WSMAN_OPTION_SET>, createxml: ::core::option::Option<*const WSMAN_DATA>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_SHELL_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SESSION_HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManCreateShell(session : WSMAN_SESSION_HANDLE, flags : u32, resourceuri : ::windows_core::PCWSTR, startupinfo : *const WSMAN_SHELL_STARTUP_INFO_V11, options : *const WSMAN_OPTION_SET, createxml : *const WSMAN_DATA, r#async : *const WSMAN_SHELL_ASYNC, shell : *mut WSMAN_SHELL_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManCreateShell(session.into_param().abi(), flags, resourceuri.into_param().abi(), ::core::mem::transmute(startupinfo.unwrap_or(::std::ptr::null())), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), ::core::mem::transmute(createxml.unwrap_or(::std::ptr::null())), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[inline]
pub unsafe fn WSManCreateShellEx<P0, P1, P2>(session: P0, flags: u32, resourceuri: P1, shellid: P2, startupinfo: ::core::option::Option<*const WSMAN_SHELL_STARTUP_INFO_V11>, options: ::core::option::Option<*const WSMAN_OPTION_SET>, createxml: ::core::option::Option<*const WSMAN_DATA>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_SHELL_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SESSION_HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManCreateShellEx(session : WSMAN_SESSION_HANDLE, flags : u32, resourceuri : ::windows_core::PCWSTR, shellid : ::windows_core::PCWSTR, startupinfo : *const WSMAN_SHELL_STARTUP_INFO_V11, options : *const WSMAN_OPTION_SET, createxml : *const WSMAN_DATA, r#async : *const WSMAN_SHELL_ASYNC, shell : *mut WSMAN_SHELL_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManCreateShellEx(session.into_param().abi(), flags, resourceuri.into_param().abi(), shellid.into_param().abi(), ::core::mem::transmute(startupinfo.unwrap_or(::std::ptr::null())), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), ::core::mem::transmute(createxml.unwrap_or(::std::ptr::null())), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[inline]
pub unsafe fn WSManDeinitialize<P0>(apihandle: P0, flags: u32) -> u32
where
    P0: ::windows_core::IntoParam<WSMAN_API_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManDeinitialize(apihandle : WSMAN_API_HANDLE, flags : u32) -> u32);
    WSManDeinitialize(apihandle.into_param().abi(), flags)
}
#[inline]
pub unsafe fn WSManDisconnectShell<P0>(shell: P0, flags: u32, disconnectinfo: *const WSMAN_SHELL_DISCONNECT_INFO, r#async: *const WSMAN_SHELL_ASYNC)
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManDisconnectShell(shell : WSMAN_SHELL_HANDLE, flags : u32, disconnectinfo : *const WSMAN_SHELL_DISCONNECT_INFO, r#async : *const WSMAN_SHELL_ASYNC));
    WSManDisconnectShell(shell.into_param().abi(), flags, disconnectinfo, r#async)
}
#[inline]
pub unsafe fn WSManGetErrorMessage<P0, P1>(apihandle: P0, flags: u32, languagecode: P1, errorcode: u32, message: ::core::option::Option<&mut [u16]>, messagelengthused: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<WSMAN_API_HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManGetErrorMessage(apihandle : WSMAN_API_HANDLE, flags : u32, languagecode : ::windows_core::PCWSTR, errorcode : u32, messagelength : u32, message : ::windows_core::PWSTR, messagelengthused : *mut u32) -> u32);
    WSManGetErrorMessage(apihandle.into_param().abi(), flags, languagecode.into_param().abi(), errorcode, message.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(message.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), messagelengthused)
}
#[inline]
pub unsafe fn WSManGetSessionOptionAsDword<P0>(session: P0, option: WSManSessionOption, value: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<WSMAN_SESSION_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManGetSessionOptionAsDword(session : WSMAN_SESSION_HANDLE, option : WSManSessionOption, value : *mut u32) -> u32);
    WSManGetSessionOptionAsDword(session.into_param().abi(), option, value)
}
#[inline]
pub unsafe fn WSManGetSessionOptionAsString<P0>(session: P0, option: WSManSessionOption, string: ::core::option::Option<&mut [u16]>, stringlengthused: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<WSMAN_SESSION_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManGetSessionOptionAsString(session : WSMAN_SESSION_HANDLE, option : WSManSessionOption, stringlength : u32, string : ::windows_core::PWSTR, stringlengthused : *mut u32) -> u32);
    WSManGetSessionOptionAsString(session.into_param().abi(), option, string.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(string.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), stringlengthused)
}
#[inline]
pub unsafe fn WSManInitialize(flags: u32, apihandle: *mut WSMAN_API_HANDLE) -> u32 {
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManInitialize(flags : u32, apihandle : *mut WSMAN_API_HANDLE) -> u32);
    WSManInitialize(flags, apihandle)
}
#[inline]
pub unsafe fn WSManPluginAuthzOperationComplete<P0>(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, userauthorizationcontext: ::core::option::Option<*const ::core::ffi::c_void>, errorcode: u32, extendederrorinformation: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginAuthzOperationComplete(senderdetails : *const WSMAN_SENDER_DETAILS, flags : u32, userauthorizationcontext : *const ::core::ffi::c_void, errorcode : u32, extendederrorinformation : ::windows_core::PCWSTR) -> u32);
    WSManPluginAuthzOperationComplete(senderdetails, flags, ::core::mem::transmute(userauthorizationcontext.unwrap_or(::std::ptr::null())), errorcode, extendederrorinformation.into_param().abi())
}
#[inline]
pub unsafe fn WSManPluginAuthzQueryQuotaComplete<P0>(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, quota: ::core::option::Option<*const WSMAN_AUTHZ_QUOTA>, errorcode: u32, extendederrorinformation: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginAuthzQueryQuotaComplete(senderdetails : *const WSMAN_SENDER_DETAILS, flags : u32, quota : *const WSMAN_AUTHZ_QUOTA, errorcode : u32, extendederrorinformation : ::windows_core::PCWSTR) -> u32);
    WSManPluginAuthzQueryQuotaComplete(senderdetails, flags, ::core::mem::transmute(quota.unwrap_or(::std::ptr::null())), errorcode, extendederrorinformation.into_param().abi())
}
#[inline]
pub unsafe fn WSManPluginAuthzUserComplete<P0, P1, P2>(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, userauthorizationcontext: ::core::option::Option<*const ::core::ffi::c_void>, impersonationtoken: P0, userisadministrator: P1, errorcode: u32, extendederrorinformation: P2) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginAuthzUserComplete(senderdetails : *const WSMAN_SENDER_DETAILS, flags : u32, userauthorizationcontext : *const ::core::ffi::c_void, impersonationtoken : super::super::Foundation:: HANDLE, userisadministrator : super::super::Foundation:: BOOL, errorcode : u32, extendederrorinformation : ::windows_core::PCWSTR) -> u32);
    WSManPluginAuthzUserComplete(senderdetails, flags, ::core::mem::transmute(userauthorizationcontext.unwrap_or(::std::ptr::null())), impersonationtoken.into_param().abi(), userisadministrator.into_param().abi(), errorcode, extendederrorinformation.into_param().abi())
}
#[inline]
pub unsafe fn WSManPluginFreeRequestDetails(requestdetails: *const WSMAN_PLUGIN_REQUEST) -> u32 {
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginFreeRequestDetails(requestdetails : *const WSMAN_PLUGIN_REQUEST) -> u32);
    WSManPluginFreeRequestDetails(requestdetails)
}
#[inline]
pub unsafe fn WSManPluginGetConfiguration(plugincontext: *const ::core::ffi::c_void, flags: u32, data: *mut WSMAN_DATA) -> u32 {
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginGetConfiguration(plugincontext : *const ::core::ffi::c_void, flags : u32, data : *mut WSMAN_DATA) -> u32);
    WSManPluginGetConfiguration(plugincontext, flags, data)
}
#[inline]
pub unsafe fn WSManPluginGetOperationParameters(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, data: *mut WSMAN_DATA) -> u32 {
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginGetOperationParameters(requestdetails : *const WSMAN_PLUGIN_REQUEST, flags : u32, data : *mut WSMAN_DATA) -> u32);
    WSManPluginGetOperationParameters(requestdetails, flags, data)
}
#[inline]
pub unsafe fn WSManPluginOperationComplete<P0>(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, errorcode: u32, extendedinformation: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginOperationComplete(requestdetails : *const WSMAN_PLUGIN_REQUEST, flags : u32, errorcode : u32, extendedinformation : ::windows_core::PCWSTR) -> u32);
    WSManPluginOperationComplete(requestdetails, flags, errorcode, extendedinformation.into_param().abi())
}
#[inline]
pub unsafe fn WSManPluginReceiveResult<P0, P1>(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, stream: P0, streamresult: ::core::option::Option<*const WSMAN_DATA>, commandstate: P1, exitcode: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginReceiveResult(requestdetails : *const WSMAN_PLUGIN_REQUEST, flags : u32, stream : ::windows_core::PCWSTR, streamresult : *const WSMAN_DATA, commandstate : ::windows_core::PCWSTR, exitcode : u32) -> u32);
    WSManPluginReceiveResult(requestdetails, flags, stream.into_param().abi(), ::core::mem::transmute(streamresult.unwrap_or(::std::ptr::null())), commandstate.into_param().abi(), exitcode)
}
#[inline]
pub unsafe fn WSManPluginReportCompletion(plugincontext: *const ::core::ffi::c_void, flags: u32) -> u32 {
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginReportCompletion(plugincontext : *const ::core::ffi::c_void, flags : u32) -> u32);
    WSManPluginReportCompletion(plugincontext, flags)
}
#[inline]
pub unsafe fn WSManPluginReportContext(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, context: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManPluginReportContext(requestdetails : *const WSMAN_PLUGIN_REQUEST, flags : u32, context : *const ::core::ffi::c_void) -> u32);
    WSManPluginReportContext(requestdetails, flags, context)
}
#[inline]
pub unsafe fn WSManReceiveShellOutput<P0, P1>(shell: P0, command: P1, flags: u32, desiredstreamset: ::core::option::Option<*const WSMAN_STREAM_ID_SET>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_OPERATION_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
    P1: ::windows_core::IntoParam<WSMAN_COMMAND_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManReceiveShellOutput(shell : WSMAN_SHELL_HANDLE, command : WSMAN_COMMAND_HANDLE, flags : u32, desiredstreamset : *const WSMAN_STREAM_ID_SET, r#async : *const WSMAN_SHELL_ASYNC, receiveoperation : *mut WSMAN_OPERATION_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManReceiveShellOutput(shell.into_param().abi(), command.into_param().abi(), flags, ::core::mem::transmute(desiredstreamset.unwrap_or(::std::ptr::null())), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[inline]
pub unsafe fn WSManReconnectShell<P0>(shell: P0, flags: u32, r#async: *const WSMAN_SHELL_ASYNC)
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManReconnectShell(shell : WSMAN_SHELL_HANDLE, flags : u32, r#async : *const WSMAN_SHELL_ASYNC));
    WSManReconnectShell(shell.into_param().abi(), flags, r#async)
}
#[inline]
pub unsafe fn WSManReconnectShellCommand<P0>(commandhandle: P0, flags: u32, r#async: *const WSMAN_SHELL_ASYNC)
where
    P0: ::windows_core::IntoParam<WSMAN_COMMAND_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManReconnectShellCommand(commandhandle : WSMAN_COMMAND_HANDLE, flags : u32, r#async : *const WSMAN_SHELL_ASYNC));
    WSManReconnectShellCommand(commandhandle.into_param().abi(), flags, r#async)
}
#[inline]
pub unsafe fn WSManRunShellCommand<P0, P1>(shell: P0, flags: u32, commandline: P1, args: ::core::option::Option<*const WSMAN_COMMAND_ARG_SET>, options: ::core::option::Option<*const WSMAN_OPTION_SET>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_COMMAND_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManRunShellCommand(shell : WSMAN_SHELL_HANDLE, flags : u32, commandline : ::windows_core::PCWSTR, args : *const WSMAN_COMMAND_ARG_SET, options : *const WSMAN_OPTION_SET, r#async : *const WSMAN_SHELL_ASYNC, command : *mut WSMAN_COMMAND_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManRunShellCommand(shell.into_param().abi(), flags, commandline.into_param().abi(), ::core::mem::transmute(args.unwrap_or(::std::ptr::null())), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[inline]
pub unsafe fn WSManRunShellCommandEx<P0, P1, P2>(shell: P0, flags: u32, commandid: P1, commandline: P2, args: ::core::option::Option<*const WSMAN_COMMAND_ARG_SET>, options: ::core::option::Option<*const WSMAN_OPTION_SET>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_COMMAND_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManRunShellCommandEx(shell : WSMAN_SHELL_HANDLE, flags : u32, commandid : ::windows_core::PCWSTR, commandline : ::windows_core::PCWSTR, args : *const WSMAN_COMMAND_ARG_SET, options : *const WSMAN_OPTION_SET, r#async : *const WSMAN_SHELL_ASYNC, command : *mut WSMAN_COMMAND_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManRunShellCommandEx(shell.into_param().abi(), flags, commandid.into_param().abi(), commandline.into_param().abi(), ::core::mem::transmute(args.unwrap_or(::std::ptr::null())), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[inline]
pub unsafe fn WSManSendShellInput<P0, P1, P2, P3>(shell: P0, command: P1, flags: u32, streamid: P2, streamdata: *const WSMAN_DATA, endofstream: P3, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_OPERATION_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
    P1: ::windows_core::IntoParam<WSMAN_COMMAND_HANDLE>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManSendShellInput(shell : WSMAN_SHELL_HANDLE, command : WSMAN_COMMAND_HANDLE, flags : u32, streamid : ::windows_core::PCWSTR, streamdata : *const WSMAN_DATA, endofstream : super::super::Foundation:: BOOL, r#async : *const WSMAN_SHELL_ASYNC, sendoperation : *mut WSMAN_OPERATION_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManSendShellInput(shell.into_param().abi(), command.into_param().abi(), flags, streamid.into_param().abi(), streamdata, endofstream.into_param().abi(), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[inline]
pub unsafe fn WSManSetSessionOption<P0>(session: P0, option: WSManSessionOption, data: *const WSMAN_DATA) -> u32
where
    P0: ::windows_core::IntoParam<WSMAN_SESSION_HANDLE>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManSetSessionOption(session : WSMAN_SESSION_HANDLE, option : WSManSessionOption, data : *const WSMAN_DATA) -> u32);
    WSManSetSessionOption(session.into_param().abi(), option, data)
}
#[inline]
pub unsafe fn WSManSignalShell<P0, P1, P2>(shell: P0, command: P1, flags: u32, code: P2, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_OPERATION_HANDLE
where
    P0: ::windows_core::IntoParam<WSMAN_SHELL_HANDLE>,
    P1: ::windows_core::IntoParam<WSMAN_COMMAND_HANDLE>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wsmsvc.dll" "system" fn WSManSignalShell(shell : WSMAN_SHELL_HANDLE, command : WSMAN_COMMAND_HANDLE, flags : u32, code : ::windows_core::PCWSTR, r#async : *const WSMAN_SHELL_ASYNC, signaloperation : *mut WSMAN_OPERATION_HANDLE));
    let mut result__ = ::std::mem::zeroed();
    WSManSignalShell(shell.into_param().abi(), command.into_param().abi(), flags, code.into_param().abi(), r#async, &mut result__);
    ::std::mem::transmute(result__)
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSMan,
    IWSMan_Vtbl,
    0x190d8637_5cd3_496d_ad24_69636bb5a3b5
);
#[cfg(feature = "Win32_System_Com")]
impl IWSMan {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0, P1>(&self, connection: P0, flags: i32, connectionoptions: P1) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSession)(::windows_core::Interface::as_raw(self), connection.into_param().abi(), flags, connectionoptions.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateConnectionOptions(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateConnectionOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CommandLine(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CommandLine)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSMan, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSMan_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connection: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, connectionoptions: *mut ::core::ffi::c_void, session: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSession: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateConnectionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateConnectionOptions: usize,
    pub CommandLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManConnectionOptions,
    IWSManConnectionOptions_Vtbl,
    0xf704e861_9e52_464f_b786_da5eb2320fdd
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptions {
    pub unsafe fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUserName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetUserName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, password: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPassword)(::windows_core::Interface::as_raw(self), password.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManConnectionOptions, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptions_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, password: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManConnectionOptionsEx,
    IWSManConnectionOptionsEx_Vtbl,
    0xef43edf7_2a48_4d93_9526_8bd6ab6d4a6b
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsEx {
    pub unsafe fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUserName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetUserName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, password: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPassword)(::windows_core::Interface::as_raw(self), password.into_param().abi()).ok()
    }
    pub unsafe fn CertificateThumbprint(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CertificateThumbprint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCertificateThumbprint<P0>(&self, thumbprint: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCertificateThumbprint)(::windows_core::Interface::as_raw(self), thumbprint.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManConnectionOptionsEx, ::windows_core::IUnknown, super::Com::IDispatch, IWSManConnectionOptions);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptionsEx_Vtbl {
    pub base__: IWSManConnectionOptions_Vtbl,
    pub CertificateThumbprint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thumbprint: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCertificateThumbprint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thumbprint: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManConnectionOptionsEx2,
    IWSManConnectionOptionsEx2_Vtbl,
    0xf500c9ec_24ee_48ab_b38d_fc9a164c658e
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsEx2 {
    pub unsafe fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUserName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetUserName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, password: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPassword)(::windows_core::Interface::as_raw(self), password.into_param().abi()).ok()
    }
    pub unsafe fn CertificateThumbprint(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CertificateThumbprint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCertificateThumbprint<P0>(&self, thumbprint: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetCertificateThumbprint)(::windows_core::Interface::as_raw(self), thumbprint.into_param().abi()).ok()
    }
    pub unsafe fn SetProxy<P0, P1>(&self, accesstype: i32, authenticationmechanism: i32, username: P0, password: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProxy)(::windows_core::Interface::as_raw(self), accesstype, authenticationmechanism, username.into_param().abi(), password.into_param().abi()).ok()
    }
    pub unsafe fn ProxyIEConfig(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProxyIEConfig)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProxyWinHttpConfig(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProxyWinHttpConfig)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProxyAutoDetect(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProxyAutoDetect)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProxyNoProxyServer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProxyNoProxyServer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProxyAuthenticationUseNegotiate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProxyAuthenticationUseNegotiate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProxyAuthenticationUseBasic(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProxyAuthenticationUseBasic)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProxyAuthenticationUseDigest(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProxyAuthenticationUseDigest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManConnectionOptionsEx2, ::windows_core::IUnknown, super::Com::IDispatch, IWSManConnectionOptions, IWSManConnectionOptionsEx);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptionsEx2_Vtbl {
    pub base__: IWSManConnectionOptionsEx_Vtbl,
    pub SetProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ProxyIEConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub ProxyWinHttpConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub ProxyAutoDetect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub ProxyNoProxyServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub ProxyAuthenticationUseNegotiate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub ProxyAuthenticationUseBasic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub ProxyAuthenticationUseDigest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManEnumerator,
    IWSManEnumerator_Vtbl,
    0xf3457ca9_abb9_4fa5_b850_90e8ca300e7f
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManEnumerator {
    pub unsafe fn ReadItem(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReadItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AtEndOfStream(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AtEndOfStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManEnumerator, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEnumerator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ReadItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AtEndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eos: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManEx,
    IWSManEx_Vtbl,
    0x2d53bdaa_798e_49e6_a1aa_74d01256f411
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0, P1>(&self, connection: P0, flags: i32, connectionoptions: P1) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSession)(::windows_core::Interface::as_raw(self), connection.into_param().abi(), flags, connectionoptions.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateConnectionOptions(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateConnectionOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CommandLine(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CommandLine)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResourceLocator<P0>(&self, strresourcelocator: P0) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateResourceLocator)(::windows_core::Interface::as_raw(self), strresourcelocator.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUTF8(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUTF8)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagCredUsernamePassword(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagCredUsernamePassword)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCACheck(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagSkipCACheck)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCNCheck(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagSkipCNCheck)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseDigest(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUseDigest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNegotiate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUseNegotiate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseBasic(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUseBasic)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseKerberos(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUseKerberos)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagNoEncryption(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagNoEncryption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagEnableSPNServerPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagEnableSPNServerPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNoAuthentication(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUseNoAuthentication)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagNonXmlText(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagNonXmlText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnEPR(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagReturnEPR)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObjectAndEPR(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagReturnObjectAndEPR)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorMessage(&self, errornumber: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorMessage)(::windows_core::Interface::as_raw(self), errornumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeep(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagHierarchyDeep)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyShallow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagHierarchyShallow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagHierarchyDeepBasePropsOnly)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObject(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagReturnObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManEx, ::windows_core::IUnknown, super::Com::IDispatch, IWSMan);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx_Vtbl {
    pub base__: IWSMan_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResourceLocator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strresourcelocator: ::std::mem::MaybeUninit<::windows_core::BSTR>, newresourcelocator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResourceLocator: usize,
    pub SessionFlagUTF8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagCredUsernamePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagSkipCACheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagSkipCNCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagUseDigest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagUseNegotiate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagUseBasic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagUseKerberos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagNoEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagEnableSPNServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagUseNoAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub EnumerationFlagNonXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub EnumerationFlagReturnEPR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub EnumerationFlagReturnObjectAndEPR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub GetErrorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errornumber: u32, errormessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EnumerationFlagHierarchyDeep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub EnumerationFlagHierarchyShallow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub EnumerationFlagHierarchyDeepBasePropsOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub EnumerationFlagReturnObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManEx2,
    IWSManEx2_Vtbl,
    0x1d1b5ae0_42d9_4021_8261_3987619512e9
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0, P1>(&self, connection: P0, flags: i32, connectionoptions: P1) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateSession)(::windows_core::Interface::as_raw(self), connection.into_param().abi(), flags, connectionoptions.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateConnectionOptions(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateConnectionOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CommandLine(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CommandLine)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResourceLocator<P0>(&self, strresourcelocator: P0) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateResourceLocator)(::windows_core::Interface::as_raw(self), strresourcelocator.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUTF8(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagUTF8)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagCredUsernamePassword(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagCredUsernamePassword)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCACheck(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagSkipCACheck)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCNCheck(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagSkipCNCheck)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseDigest(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagUseDigest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNegotiate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagUseNegotiate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseBasic(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagUseBasic)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseKerberos(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagUseKerberos)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagNoEncryption(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagNoEncryption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagEnableSPNServerPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagEnableSPNServerPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNoAuthentication(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagUseNoAuthentication)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagNonXmlText(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerationFlagNonXmlText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnEPR(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerationFlagReturnEPR)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObjectAndEPR(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerationFlagReturnObjectAndEPR)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorMessage(&self, errornumber: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetErrorMessage)(::windows_core::Interface::as_raw(self), errornumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeep(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerationFlagHierarchyDeep)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyShallow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerationFlagHierarchyShallow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerationFlagHierarchyDeepBasePropsOnly)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObject(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerationFlagReturnObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseClientCertificate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUseClientCertificate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManEx2, ::windows_core::IUnknown, super::Com::IDispatch, IWSMan, IWSManEx);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx2_Vtbl {
    pub base__: IWSManEx_Vtbl,
    pub SessionFlagUseClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManEx3,
    IWSManEx3_Vtbl,
    0x6400e966_011d_4eac_8474_049e0848afad
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx3 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0, P1>(&self, connection: P0, flags: i32, connectionoptions: P1) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateSession)(::windows_core::Interface::as_raw(self), connection.into_param().abi(), flags, connectionoptions.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateConnectionOptions(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateConnectionOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CommandLine(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CommandLine)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResourceLocator<P0>(&self, strresourcelocator: P0) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateResourceLocator)(::windows_core::Interface::as_raw(self), strresourcelocator.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUTF8(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagUTF8)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagCredUsernamePassword(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagCredUsernamePassword)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCACheck(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagSkipCACheck)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCNCheck(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagSkipCNCheck)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseDigest(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagUseDigest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNegotiate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagUseNegotiate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseBasic(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagUseBasic)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseKerberos(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagUseKerberos)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagNoEncryption(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagNoEncryption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagEnableSPNServerPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagEnableSPNServerPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNoAuthentication(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SessionFlagUseNoAuthentication)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagNonXmlText(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumerationFlagNonXmlText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnEPR(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumerationFlagReturnEPR)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObjectAndEPR(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumerationFlagReturnObjectAndEPR)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorMessage(&self, errornumber: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetErrorMessage)(::windows_core::Interface::as_raw(self), errornumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeep(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumerationFlagHierarchyDeep)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyShallow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumerationFlagHierarchyShallow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumerationFlagHierarchyDeepBasePropsOnly)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObject(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumerationFlagReturnObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseClientCertificate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionFlagUseClientCertificate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUTF16(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUTF16)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseCredSsp(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUseCredSsp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagAssociationInstance(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagAssociationInstance)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagAssociatedInstance(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerationFlagAssociatedInstance)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipRevocationCheck(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagSkipRevocationCheck)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagAllowNegotiateImplicitCredentials(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagAllowNegotiateImplicitCredentials)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseSsl(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionFlagUseSsl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManEx3, ::windows_core::IUnknown, super::Com::IDispatch, IWSMan, IWSManEx, IWSManEx2);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx3_Vtbl {
    pub base__: IWSManEx2_Vtbl,
    pub SessionFlagUTF16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagUseCredSsp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub EnumerationFlagAssociationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub EnumerationFlagAssociatedInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagSkipRevocationCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagAllowNegotiateImplicitCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
    pub SessionFlagUseSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManInternal,
    IWSManInternal_Vtbl,
    0x04ae2b1d_9954_4d99_94a9_a961e72c3a13
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManInternal {
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConfigSDDL<P0>(&self, session: P0, resourceuri: super::Variant::VARIANT, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConfigSDDL)(::windows_core::Interface::as_raw(self), session.into_param().abi(), ::core::mem::transmute(resourceuri), flags, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManInternal, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManInternal_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ConfigSDDL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32, resource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ConfigSDDL: usize,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManResourceLocator,
    IWSManResourceLocator_Vtbl,
    0xa7a1ba28_de41_466a_ad0a_c4059ead7428
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManResourceLocator {
    pub unsafe fn SetResourceURI<P0>(&self, uri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetResourceURI)(::windows_core::Interface::as_raw(self), uri.into_param().abi()).ok()
    }
    pub unsafe fn ResourceURI(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ResourceURI)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddSelector<P0>(&self, resourceselname: P0, selvalue: super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AddSelector)(::windows_core::Interface::as_raw(self), resourceselname.into_param().abi(), ::core::mem::transmute(selvalue)).ok()
    }
    pub unsafe fn ClearSelectors(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearSelectors)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FragmentPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FragmentPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFragmentPath<P0>(&self, text: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFragmentPath)(::windows_core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    pub unsafe fn FragmentDialect(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FragmentDialect)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFragmentDialect<P0>(&self, text: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFragmentDialect)(::windows_core::Interface::as_raw(self), text.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddOption<P0, P1>(&self, optionname: P0, optionvalue: super::Variant::VARIANT, mustcomply: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).AddOption)(::windows_core::Interface::as_raw(self), optionname.into_param().abi(), ::core::mem::transmute(optionvalue), mustcomply.into_param().abi()).ok()
    }
    pub unsafe fn SetMustUnderstandOptions<P0>(&self, mustunderstand: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetMustUnderstandOptions)(::windows_core::Interface::as_raw(self), mustunderstand.into_param().abi()).ok()
    }
    pub unsafe fn MustUnderstandOptions(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MustUnderstandOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClearOptions(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearOptions)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Error(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManResourceLocator, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManResourceLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetResourceURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ResourceURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceselname: ::std::mem::MaybeUninit<::windows_core::BSTR>, selvalue: super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddSelector: usize,
    pub ClearSelectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FragmentPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFragmentPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FragmentDialect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFragmentDialect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionvalue: super::Variant::VARIANT, mustcomply: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddOption: usize,
    pub SetMustUnderstandOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mustunderstand: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub MustUnderstandOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mustunderstand: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub ClearOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IWSManResourceLocatorInternal, IWSManResourceLocatorInternal_Vtbl, 0xeffaead7_7ec8_4716_b9be_f2e7e9fb4adb);
impl IWSManResourceLocatorInternal {}
::windows_core::imp::interface_hierarchy!(IWSManResourceLocatorInternal, ::windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IWSManResourceLocatorInternal_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    IWSManSession,
    IWSManSession_Vtbl,
    0xfc84fc58_1286_40c4_9da0_c8ef6ec241e0
);
#[cfg(feature = "Win32_System_Com")]
impl IWSManSession {
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get(&self, resourceuri: super::Variant::VARIANT, flags: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resourceuri), flags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, resourceuri: super::Variant::VARIANT, resource: P0, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Put)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resourceuri), resource.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Create<P0>(&self, resourceuri: super::Variant::VARIANT, resource: P0, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resourceuri), resource.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Delete(&self, resourceuri: super::Variant::VARIANT, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resourceuri), flags).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Invoke2<P0, P1>(&self, actionuri: P0, resourceuri: super::Variant::VARIANT, parameters: P1, flags: i32) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Invoke2)(::windows_core::Interface::as_raw(self), actionuri.into_param().abi(), ::core::mem::transmute(resourceuri), parameters.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Enumerate<P0, P1>(&self, resourceuri: super::Variant::VARIANT, filter: P0, dialect: P1, flags: i32) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enumerate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resourceuri), filter.into_param().abi(), dialect.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Identify(&self, flags: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Identify)(::windows_core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BatchItems(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BatchItems)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBatchItems(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBatchItems)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Timeout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Timeout)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTimeout(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTimeout)(::windows_core::Interface::as_raw(self), value).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWSManSession, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32, resource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Get: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, resource: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, resultresource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Put: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, resource: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, newuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Create: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Delete: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Invoke2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actionuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, resourceuri: super::Variant::VARIANT, parameters: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, result: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Invoke2: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Enumerate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceuri: super::Variant::VARIANT, filter: ::std::mem::MaybeUninit<::windows_core::BSTR>, dialect: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, resultset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Enumerate: usize,
    pub Identify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, result: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BatchItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub SetBatchItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Timeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
pub const ERROR_REDIRECT_LOCATION_INVALID: u32 = 2150859191u32;
pub const ERROR_REDIRECT_LOCATION_TOO_LONG: u32 = 2150859190u32;
pub const ERROR_SERVICE_CBT_HARDENING_INVALID: u32 = 2150859192u32;
pub const ERROR_WINRS_CLIENT_CLOSERECEIVEHANDLE_NULL_PARAM: u32 = 2150859058u32;
pub const ERROR_WINRS_CLIENT_CLOSESENDHANDLE_NULL_PARAM: u32 = 2150859061u32;
pub const ERROR_WINRS_CLIENT_CLOSESHELL_NULL_PARAM: u32 = 2150859050u32;
pub const ERROR_WINRS_CLIENT_CREATESHELL_NULL_PARAM: u32 = 2150859049u32;
pub const ERROR_WINRS_CLIENT_FREECREATESHELLRESULT_NULL_PARAM: u32 = 2150859051u32;
pub const ERROR_WINRS_CLIENT_FREEPULLRESULT_NULL_PARAM: u32 = 2150859056u32;
pub const ERROR_WINRS_CLIENT_FREERUNCOMMANDRESULT_NULL_PARAM: u32 = 2150859053u32;
pub const ERROR_WINRS_CLIENT_GET_NULL_PARAM: u32 = 2150859062u32;
pub const ERROR_WINRS_CLIENT_INVALID_FLAG: u32 = 2150859040u32;
pub const ERROR_WINRS_CLIENT_NULL_PARAM: u32 = 2150859041u32;
pub const ERROR_WINRS_CLIENT_PULL_NULL_PARAM: u32 = 2150859057u32;
pub const ERROR_WINRS_CLIENT_PUSH_NULL_PARAM: u32 = 2150859060u32;
pub const ERROR_WINRS_CLIENT_RECEIVE_NULL_PARAM: u32 = 2150859055u32;
pub const ERROR_WINRS_CLIENT_RUNCOMMAND_NULL_PARAM: u32 = 2150859052u32;
pub const ERROR_WINRS_CLIENT_SEND_NULL_PARAM: u32 = 2150859059u32;
pub const ERROR_WINRS_CLIENT_SIGNAL_NULL_PARAM: u32 = 2150859054u32;
pub const ERROR_WINRS_CODE_PAGE_NOT_SUPPORTED: u32 = 2150859072u32;
pub const ERROR_WINRS_CONNECT_RESPONSE_BAD_BODY: u32 = 2150859211u32;
pub const ERROR_WINRS_IDLETIMEOUT_OUTOFBOUNDS: u32 = 2150859250u32;
pub const ERROR_WINRS_RECEIVE_IN_PROGRESS: u32 = 2150859047u32;
pub const ERROR_WINRS_RECEIVE_NO_RESPONSE_DATA: u32 = 2150859048u32;
pub const ERROR_WINRS_SHELLCOMMAND_CLIENTID_NOT_VALID: u32 = 2150859220u32;
pub const ERROR_WINRS_SHELLCOMMAND_CLIENTID_RESOURCE_CONFLICT: u32 = 2150859222u32;
pub const ERROR_WINRS_SHELLCOMMAND_DISCONNECT_OPERATION_NOT_VALID: u32 = 2150859224u32;
pub const ERROR_WINRS_SHELLCOMMAND_RECONNECT_OPERATION_NOT_VALID: u32 = 2150859219u32;
pub const ERROR_WINRS_SHELL_CLIENTID_NOT_VALID: u32 = 2150859221u32;
pub const ERROR_WINRS_SHELL_CLIENTID_RESOURCE_CONFLICT: u32 = 2150859223u32;
pub const ERROR_WINRS_SHELL_CLIENTSESSIONID_MISMATCH: u32 = 2150859206u32;
pub const ERROR_WINRS_SHELL_CONNECTED_TO_DIFFERENT_CLIENT: u32 = 2150859213u32;
pub const ERROR_WINRS_SHELL_DISCONNECTED: u32 = 2150859204u32;
pub const ERROR_WINRS_SHELL_DISCONNECT_NOT_SUPPORTED: u32 = 2150859205u32;
pub const ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_GRACEFUL: u32 = 2150859214u32;
pub const ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_VALID: u32 = 2150859215u32;
pub const ERROR_WINRS_SHELL_RECONNECT_OPERATION_NOT_VALID: u32 = 2150859216u32;
pub const ERROR_WINRS_SHELL_URI_INVALID: u32 = 2150859099u32;
pub const ERROR_WSMAN_ACK_NOT_SUPPORTED: u32 = 2150858853u32;
pub const ERROR_WSMAN_ACTION_MISMATCH: u32 = 2150858801u32;
pub const ERROR_WSMAN_ACTION_NOT_SUPPORTED: u32 = 2150858771u32;
pub const ERROR_WSMAN_ADDOBJECT_MISSING_EPR: u32 = 2150859045u32;
pub const ERROR_WSMAN_ADDOBJECT_MISSING_OBJECT: u32 = 2150859044u32;
pub const ERROR_WSMAN_ALREADY_EXISTS: u32 = 2150858803u32;
pub const ERROR_WSMAN_AMBIGUOUS_SELECTORS: u32 = 2150858846u32;
pub const ERROR_WSMAN_AUTHENTICATION_INVALID_FLAG: u32 = 2150859077u32;
pub const ERROR_WSMAN_AUTHORIZATION_MODE_NOT_SUPPORTED: u32 = 2150858852u32;
pub const ERROR_WSMAN_BAD_METHOD: u32 = 2150858868u32;
pub const ERROR_WSMAN_BATCHSIZE_TOO_SMALL: u32 = 2150858919u32;
pub const ERROR_WSMAN_BATCH_COMPLETE: u32 = 2150858756u32;
pub const ERROR_WSMAN_BOOKMARKS_NOT_SUPPORTED: u32 = 2150858859u32;
pub const ERROR_WSMAN_BOOKMARK_EXPIRED: u32 = 2150858832u32;
pub const ERROR_WSMAN_CANNOT_CHANGE_KEYS: u32 = 2150858989u32;
pub const ERROR_WSMAN_CANNOT_DECRYPT: u32 = 2150859001u32;
pub const ERROR_WSMAN_CANNOT_PROCESS_FILTER: u32 = 2150859042u32;
pub const ERROR_WSMAN_CANNOT_USE_ALLOW_NEGOTIATE_IMPLICIT_CREDENTIALS_FOR_HTTP: u32 = 2150859184u32;
pub const ERROR_WSMAN_CANNOT_USE_CERTIFICATES_FOR_HTTP: u32 = 2150858968u32;
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_CREDSSP: u32 = 2150859187u32;
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_HTTP: u32 = 2150859185u32;
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_KERBEROS: u32 = 2150859186u32;
pub const ERROR_WSMAN_CERTMAPPING_CONFIGLIMIT_EXCEEDED: u32 = 2150859091u32;
pub const ERROR_WSMAN_CERTMAPPING_CREDENTIAL_MANAGEMENT_FAILIED: u32 = 2150859262u32;
pub const ERROR_WSMAN_CERTMAPPING_INVALIDISSUERKEY: u32 = 2150859106u32;
pub const ERROR_WSMAN_CERTMAPPING_INVALIDSUBJECTKEY: u32 = 2150859105u32;
pub const ERROR_WSMAN_CERTMAPPING_INVALIDUSERCREDENTIALS: u32 = 2150859092u32;
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDBLANK: u32 = 2150859115u32;
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDTOOLONG: u32 = 2150859114u32;
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDUSERTUPLE: u32 = 2150859116u32;
pub const ERROR_WSMAN_CERT_INVALID_USAGE: u32 = 2150858990u32;
pub const ERROR_WSMAN_CERT_INVALID_USAGE_CLIENT: u32 = 2150859093u32;
pub const ERROR_WSMAN_CERT_MISSING_AUTH_FLAG: u32 = 2150859094u32;
pub const ERROR_WSMAN_CERT_MULTIPLE_CREDENTIALS_FLAG: u32 = 2150859095u32;
pub const ERROR_WSMAN_CERT_NOT_FOUND: u32 = 2150858882u32;
pub const ERROR_WSMAN_CERT_THUMBPRINT_BLANK: u32 = 2150858983u32;
pub const ERROR_WSMAN_CERT_THUMBPRINT_NOT_BLANK: u32 = 2150858982u32;
pub const ERROR_WSMAN_CHARACTER_SET: u32 = 2150858828u32;
pub const ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS: u32 = 2150859171u32;
pub const ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS_NTLMONLY: u32 = 2150859172u32;
pub const ERROR_WSMAN_CLIENT_BASIC_AUTHENTICATION_DISABLED: u32 = 2150858975u32;
pub const ERROR_WSMAN_CLIENT_BATCH_ITEMS_TOO_SMALL: u32 = 2150858946u32;
pub const ERROR_WSMAN_CLIENT_BLANK_ACTION_URI: u32 = 2150858948u32;
pub const ERROR_WSMAN_CLIENT_BLANK_INPUT_XML: u32 = 2150858945u32;
pub const ERROR_WSMAN_CLIENT_BLANK_URI: u32 = 2150858943u32;
pub const ERROR_WSMAN_CLIENT_CERTIFICATES_AUTHENTICATION_DISABLED: u32 = 2150858979u32;
pub const ERROR_WSMAN_CLIENT_CERT_NEEDED: u32 = 2150858932u32;
pub const ERROR_WSMAN_CLIENT_CERT_UNKNOWN_LOCATION: u32 = 2150858934u32;
pub const ERROR_WSMAN_CLIENT_CERT_UNKNOWN_TYPE: u32 = 2150858933u32;
pub const ERROR_WSMAN_CLIENT_CERT_UNNEEDED_CREDS: u32 = 2150858927u32;
pub const ERROR_WSMAN_CLIENT_CERT_UNNEEDED_USERNAME: u32 = 2150858929u32;
pub const ERROR_WSMAN_CLIENT_CLOSECOMMAND_NULL_PARAM: u32 = 2150859135u32;
pub const ERROR_WSMAN_CLIENT_CLOSESHELL_NULL_PARAM: u32 = 2150859134u32;
pub const ERROR_WSMAN_CLIENT_COMPRESSION_INVALID_OPTION: u32 = 2150858957u32;
pub const ERROR_WSMAN_CLIENT_CONNECTCOMMAND_NULL_PARAM: u32 = 2150859210u32;
pub const ERROR_WSMAN_CLIENT_CONNECTSHELL_NULL_PARAM: u32 = 2150859209u32;
pub const ERROR_WSMAN_CLIENT_CONSTRUCTERROR_NULL_PARAM: u32 = 2150858965u32;
pub const ERROR_WSMAN_CLIENT_CREATESESSION_NULL_PARAM: u32 = 2150858938u32;
pub const ERROR_WSMAN_CLIENT_CREATESHELL_NAME_INVALID: u32 = 2150859202u32;
pub const ERROR_WSMAN_CLIENT_CREATESHELL_NULL_PARAM: u32 = 2150859130u32;
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FLAG_NEEDED: u32 = 2150858931u32;
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_DEFAULT_AUTHENTICATION: u32 = 2150859078u32;
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_PROXY_AUTHENTICATION: u32 = 2150859163u32;
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_NEEDED: u32 = 2150858930u32;
pub const ERROR_WSMAN_CLIENT_CREDSSP_AUTHENTICATION_DISABLED: u32 = 2150859170u32;
pub const ERROR_WSMAN_CLIENT_DECODEOBJECT_NULL_PARAM: u32 = 2150858961u32;
pub const ERROR_WSMAN_CLIENT_DELIVERENDSUBSCRIPTION_NULL_PARAM: u32 = 2150858958u32;
pub const ERROR_WSMAN_CLIENT_DELIVEREVENTS_NULL_PARAM: u32 = 2150858959u32;
pub const ERROR_WSMAN_CLIENT_DIGEST_AUTHENTICATION_DISABLED: u32 = 2150858976u32;
pub const ERROR_WSMAN_CLIENT_DISABLE_LOOPBACK_WITH_EXPLICIT_CREDENTIALS: u32 = 2150859073u32;
pub const ERROR_WSMAN_CLIENT_DISCONNECTSHELL_NULL_PARAM: u32 = 2150859207u32;
pub const ERROR_WSMAN_CLIENT_ENCODEOBJECT_NULL_PARAM: u32 = 2150858962u32;
pub const ERROR_WSMAN_CLIENT_ENUMERATE_NULL_PARAM: u32 = 2150858939u32;
pub const ERROR_WSMAN_CLIENT_ENUMERATORADDEVENT_NULL_PARAM: u32 = 2150859043u32;
pub const ERROR_WSMAN_CLIENT_ENUMERATORADDOBJECT_NULL_PARAM: u32 = 2150858963u32;
pub const ERROR_WSMAN_CLIENT_ENUMERATORNEXTOBJECT_NULL_PARAM: u32 = 2150858964u32;
pub const ERROR_WSMAN_CLIENT_ENUM_RECEIVED_TOO_MANY_ITEMS: u32 = 2150859075u32;
pub const ERROR_WSMAN_CLIENT_GETBOOKMARK_NULL_PARAM: u32 = 2150858960u32;
pub const ERROR_WSMAN_CLIENT_GETERRORMESSAGE_NULL_PARAM: u32 = 2150859158u32;
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_INVALID_PARAM: u32 = 2150859167u32;
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_NULL_PARAM: u32 = 2150859166u32;
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_INVALID_PARAM: u32 = 2150859129u32;
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_STRING_INVALID_PARAM: u32 = 2150859168u32;
pub const ERROR_WSMAN_CLIENT_INITIALIZE_NULL_PARAM: u32 = 2150859124u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CERT: u32 = 2150858935u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CERT_DNS_OR_UPN: u32 = 2150859080u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CLOSE_COMMAND_FLAG: u32 = 2150859133u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CLOSE_SHELL_FLAG: u32 = 2150859132u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CREATE_SHELL_FLAG: u32 = 2150859131u32;
pub const ERROR_WSMAN_CLIENT_INVALID_DEINIT_APPLICATION_FLAG: u32 = 2150859126u32;
pub const ERROR_WSMAN_CLIENT_INVALID_DELIVERY_RETRY: u32 = 2150859108u32;
pub const ERROR_WSMAN_CLIENT_INVALID_DISABLE_LOOPBACK: u32 = 2150859074u32;
pub const ERROR_WSMAN_CLIENT_INVALID_DISCONNECT_SHELL_FLAG: u32 = 2150859226u32;
pub const ERROR_WSMAN_CLIENT_INVALID_FLAG: u32 = 2150858924u32;
pub const ERROR_WSMAN_CLIENT_INVALID_GETERRORMESSAGE_FLAG: u32 = 2150859160u32;
pub const ERROR_WSMAN_CLIENT_INVALID_INIT_APPLICATION_FLAG: u32 = 2150859125u32;
pub const ERROR_WSMAN_CLIENT_INVALID_LANGUAGE_CODE: u32 = 2150859159u32;
pub const ERROR_WSMAN_CLIENT_INVALID_LOCALE: u32 = 2150859156u32;
pub const ERROR_WSMAN_CLIENT_INVALID_RECEIVE_SHELL_FLAG: u32 = 2150859150u32;
pub const ERROR_WSMAN_CLIENT_INVALID_RESOURCE_LOCATOR: u32 = 2150858944u32;
pub const ERROR_WSMAN_CLIENT_INVALID_RUNCOMMAND_FLAG: u32 = 2150859137u32;
pub const ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_FLAG: u32 = 2150859145u32;
pub const ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_PARAMETER: u32 = 2150859146u32;
pub const ERROR_WSMAN_CLIENT_INVALID_SHELL_COMMAND_PAIR: u32 = 2150859227u32;
pub const ERROR_WSMAN_CLIENT_INVALID_SIGNAL_SHELL_FLAG: u32 = 2150859143u32;
pub const ERROR_WSMAN_CLIENT_INVALID_UI_LANGUAGE: u32 = 2150859157u32;
pub const ERROR_WSMAN_CLIENT_KERBEROS_AUTHENTICATION_DISABLED: u32 = 2150858978u32;
pub const ERROR_WSMAN_CLIENT_LOCAL_INVALID_CONNECTION_OPTIONS: u32 = 2150858937u32;
pub const ERROR_WSMAN_CLIENT_LOCAL_INVALID_CREDS: u32 = 2150858936u32;
pub const ERROR_WSMAN_CLIENT_MAX_CHARS_TOO_SMALL: u32 = 2150858947u32;
pub const ERROR_WSMAN_CLIENT_MISSING_EXPIRATION: u32 = 2150858953u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_AUTH_FLAGS: u32 = 2150858925u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_DELIVERY_MODES: u32 = 2150858950u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_ENUM_MODE_FLAGS: u32 = 2150859039u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_ENVELOPE_POLICIES: u32 = 2150858951u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_PROXY_AUTH_FLAGS: u32 = 2150859188u32;
pub const ERROR_WSMAN_CLIENT_NEGOTIATE_AUTHENTICATION_DISABLED: u32 = 2150858977u32;
pub const ERROR_WSMAN_CLIENT_NO_HANDLE: u32 = 2150858942u32;
pub const ERROR_WSMAN_CLIENT_NO_SOURCES: u32 = 2150859111u32;
pub const ERROR_WSMAN_CLIENT_NULL_ISSUERS: u32 = 2150859110u32;
pub const ERROR_WSMAN_CLIENT_NULL_PUBLISHERS: u32 = 2150859109u32;
pub const ERROR_WSMAN_CLIENT_NULL_RESULT_PARAM: u32 = 2150858941u32;
pub const ERROR_WSMAN_CLIENT_PULL_INVALID_FLAGS: u32 = 2150858954u32;
pub const ERROR_WSMAN_CLIENT_PUSH_HOST_TOO_LONG: u32 = 2150858956u32;
pub const ERROR_WSMAN_CLIENT_PUSH_UNSUPPORTED_TRANSPORT: u32 = 2150858955u32;
pub const ERROR_WSMAN_CLIENT_RECEIVE_NULL_PARAM: u32 = 2150859148u32;
pub const ERROR_WSMAN_CLIENT_RECONNECTSHELLCOMMAND_NULL_PARAM: u32 = 2150859218u32;
pub const ERROR_WSMAN_CLIENT_RECONNECTSHELL_NULL_PARAM: u32 = 2150859208u32;
pub const ERROR_WSMAN_CLIENT_RUNCOMMAND_NOTCOMPLETED: u32 = 2150859138u32;
pub const ERROR_WSMAN_CLIENT_RUNCOMMAND_NULL_PARAM: u32 = 2150859136u32;
pub const ERROR_WSMAN_CLIENT_SEND_NULL_PARAM: u32 = 2150859144u32;
pub const ERROR_WSMAN_CLIENT_SESSION_UNUSABLE: u32 = 2150859258u32;
pub const ERROR_WSMAN_CLIENT_SETSESSIONOPTION_INVALID_PARAM: u32 = 2150859128u32;
pub const ERROR_WSMAN_CLIENT_SETSESSIONOPTION_NULL_PARAM: u32 = 2150859127u32;
pub const ERROR_WSMAN_CLIENT_SIGNAL_NULL_PARAM: u32 = 2150859142u32;
pub const ERROR_WSMAN_CLIENT_SPN_WRONG_AUTH: u32 = 2150858926u32;
pub const ERROR_WSMAN_CLIENT_SUBSCRIBE_NULL_PARAM: u32 = 2150858940u32;
pub const ERROR_WSMAN_CLIENT_UNENCRYPTED_DISABLED: u32 = 2150858974u32;
pub const ERROR_WSMAN_CLIENT_UNENCRYPTED_HTTP_ONLY: u32 = 2150858967u32;
pub const ERROR_WSMAN_CLIENT_UNKNOWN_EXPIRATION_TYPE: u32 = 2150858952u32;
pub const ERROR_WSMAN_CLIENT_USERNAME_AND_PASSWORD_NEEDED: u32 = 2150859079u32;
pub const ERROR_WSMAN_CLIENT_USERNAME_PASSWORD_NEEDED: u32 = 2150858928u32;
pub const ERROR_WSMAN_CLIENT_WORKGROUP_NO_KERBEROS: u32 = 2150859020u32;
pub const ERROR_WSMAN_CLIENT_ZERO_HEARTBEAT: u32 = 2150858949u32;
pub const ERROR_WSMAN_COMMAND_ALREADY_CLOSED: u32 = 2150859087u32;
pub const ERROR_WSMAN_COMMAND_TERMINATED: u32 = 2150859212u32;
pub const ERROR_WSMAN_CONCURRENCY: u32 = 2150858802u32;
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_CERTMAPPING_KEYS: u32 = 2150859122u32;
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_GPO_CONTROLLED_SETTING: u32 = 2150858890u32;
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_MUTUAL: u32 = 2150858885u32;
pub const ERROR_WSMAN_CONFIG_CANNOT_SHARE_SSL_CONFIG: u32 = 2150858984u32;
pub const ERROR_WSMAN_CONFIG_CERT_CN_DOES_NOT_MATCH_HOSTNAME: u32 = 2150858985u32;
pub const ERROR_WSMAN_CONFIG_CORRUPTED: u32 = 2150858757u32;
pub const ERROR_WSMAN_CONFIG_GROUP_POLICY_CHANGE_NOTIFICATION_SUBSCRIPTION_FAILED: u32 = 2150859217u32;
pub const ERROR_WSMAN_CONFIG_HOSTNAME_CHANGE_WITHOUT_CERT: u32 = 2150858986u32;
pub const ERROR_WSMAN_CONFIG_PORT_INVALID: u32 = 2150858972u32;
pub const ERROR_WSMAN_CONFIG_READONLY_PROPERTY: u32 = 2150859071u32;
pub const ERROR_WSMAN_CONFIG_SHELLURI_INVALID_OPERATION_ON_KEY: u32 = 2150859119u32;
pub const ERROR_WSMAN_CONFIG_SHELLURI_INVALID_PROCESSPATH: u32 = 2150859098u32;
pub const ERROR_WSMAN_CONFIG_SHELL_URI_CMDSHELLURI_NOTPERMITTED: u32 = 2150859097u32;
pub const ERROR_WSMAN_CONFIG_SHELL_URI_INVALID: u32 = 2150859096u32;
pub const ERROR_WSMAN_CONFIG_THUMBPRINT_SHOULD_BE_EMPTY: u32 = 2150858987u32;
pub const ERROR_WSMAN_CONNECTIONSTR_INVALID: u32 = 2150858969u32;
pub const ERROR_WSMAN_CONNECTOR_GET: u32 = 2150858873u32;
pub const ERROR_WSMAN_CREATESHELL_NULL_ENVIRONMENT_VARIABLE_NAME: u32 = 2150859081u32;
pub const ERROR_WSMAN_CREATESHELL_NULL_STREAMID: u32 = 2150859083u32;
pub const ERROR_WSMAN_CREATESHELL_RUNAS_FAILED: u32 = 2150859231u32;
pub const ERROR_WSMAN_CREATE_RESPONSE_NO_EPR: u32 = 2150858992u32;
pub const ERROR_WSMAN_CREDSSP_USERNAME_PASSWORD_NEEDED: u32 = 2150859169u32;
pub const ERROR_WSMAN_CREDS_PASSED_WITH_NO_AUTH_FLAG: u32 = 2150858923u32;
pub const ERROR_WSMAN_CUSTOMREMOTESHELL_DEPRECATED: u32 = 2150859196u32;
pub const ERROR_WSMAN_DEFAULTAUTH_IPADDRESS: u32 = 2150859195u32;
pub const ERROR_WSMAN_DELIVERY_REFUSED: u32 = 2150858804u32;
pub const ERROR_WSMAN_DELIVERY_RETRIES_NOT_SUPPORTED: u32 = 2150858857u32;
pub const ERROR_WSMAN_DELIVER_IN_PROGRESS: u32 = 2150858821u32;
pub const ERROR_WSMAN_DEPRECATED_CONFIG_SETTING: u32 = 2150859182u32;
pub const ERROR_WSMAN_DESERIALIZE_CLASS: u32 = 2150859244u32;
pub const ERROR_WSMAN_DESTINATION_INVALID: u32 = 2150859256u32;
pub const ERROR_WSMAN_DESTINATION_UNREACHABLE: u32 = 2150858770u32;
pub const ERROR_WSMAN_DIFFERENT_AUTHZ_TOKEN: u32 = 2150859177u32;
pub const ERROR_WSMAN_DIFFERENT_CIM_SELECTOR: u32 = 2150859067u32;
pub const ERROR_WSMAN_DUPLICATE_SELECTORS: u32 = 2150858847u32;
pub const ERROR_WSMAN_ENCODING_LIMIT: u32 = 2150858805u32;
pub const ERROR_WSMAN_ENCODING_TYPE: u32 = 2150859033u32;
pub const ERROR_WSMAN_ENDPOINT_UNAVAILABLE: u32 = 2150858772u32;
pub const ERROR_WSMAN_ENDPOINT_UNAVAILABLE_INVALID_VALUE: u32 = 2150859034u32;
pub const ERROR_WSMAN_ENUMERATE_CANNOT_PROCESS_FILTER: u32 = 2150858778u32;
pub const ERROR_WSMAN_ENUMERATE_FILTERING_NOT_SUPPORTED: u32 = 2150858776u32;
pub const ERROR_WSMAN_ENUMERATE_FILTER_DIALECT_REQUESTED_UNAVAILABLE: u32 = 2150858777u32;
pub const ERROR_WSMAN_ENUMERATE_INVALID_ENUMERATION_CONTEXT: u32 = 2150858779u32;
pub const ERROR_WSMAN_ENUMERATE_INVALID_EXPIRATION_TIME: u32 = 2150858774u32;
pub const ERROR_WSMAN_ENUMERATE_SHELLCOMAMNDS_FILTER_EXPECTED: u32 = 2150859200u32;
pub const ERROR_WSMAN_ENUMERATE_SHELLCOMMANDS_EPRS_NOTSUPPORTED: u32 = 2150859201u32;
pub const ERROR_WSMAN_ENUMERATE_TIMED_OUT: u32 = 2150858780u32;
pub const ERROR_WSMAN_ENUMERATE_UNABLE_TO_RENEW: u32 = 2150858781u32;
pub const ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TIME: u32 = 2150858775u32;
pub const ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TYPE: u32 = 2150859036u32;
pub const ERROR_WSMAN_ENUMERATE_WMI_INVALID_KEY: u32 = 2150859016u32;
pub const ERROR_WSMAN_ENUMERATION_CLOSED: u32 = 2150858759u32;
pub const ERROR_WSMAN_ENUMERATION_INITIALIZING: u32 = 2150858872u32;
pub const ERROR_WSMAN_ENUMERATION_INVALID: u32 = 2150858884u32;
pub const ERROR_WSMAN_ENUMERATION_MODE_UNSUPPORTED: u32 = 2150858886u32;
pub const ERROR_WSMAN_ENVELOPE_TOO_LARGE: u32 = 2150858790u32;
pub const ERROR_WSMAN_EPR_NESTING_EXCEEDED: u32 = 2150858879u32;
pub const ERROR_WSMAN_EVENTING_CONCURRENT_CLIENT_RECEIVE: u32 = 2150858891u32;
pub const ERROR_WSMAN_EVENTING_DELIVERYFAILED_FROMSOURCE: u32 = 2150858908u32;
pub const ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_INVALID: u32 = 2150858920u32;
pub const ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_UNAVAILABLE: u32 = 2150858782u32;
pub const ERROR_WSMAN_EVENTING_FAST_SENDER: u32 = 2150858892u32;
pub const ERROR_WSMAN_EVENTING_FILTERING_NOT_SUPPORTED: u32 = 2150858785u32;
pub const ERROR_WSMAN_EVENTING_FILTERING_REQUESTED_UNAVAILABLE: u32 = 2150858786u32;
pub const ERROR_WSMAN_EVENTING_INCOMPATIBLE_BATCHPARAMS_AND_DELIVERYMODE: u32 = 2150858900u32;
pub const ERROR_WSMAN_EVENTING_INSECURE_PUSHSUBSCRIPTION_CONNECTION: u32 = 2150858893u32;
pub const ERROR_WSMAN_EVENTING_INVALID_ENCODING_IN_DELIVERY: u32 = 2150859255u32;
pub const ERROR_WSMAN_EVENTING_INVALID_ENDTO_ADDRESSS: u32 = 2150858902u32;
pub const ERROR_WSMAN_EVENTING_INVALID_EVENTSOURCE: u32 = 2150858894u32;
pub const ERROR_WSMAN_EVENTING_INVALID_EXPIRATION_TIME: u32 = 2150858783u32;
pub const ERROR_WSMAN_EVENTING_INVALID_HEARTBEAT: u32 = 2150858916u32;
pub const ERROR_WSMAN_EVENTING_INVALID_INCOMING_EVENT_PACKET_HEADER: u32 = 2150858903u32;
pub const ERROR_WSMAN_EVENTING_INVALID_LOCALE_IN_DELIVERY: u32 = 2150858915u32;
pub const ERROR_WSMAN_EVENTING_INVALID_MESSAGE: u32 = 2150858789u32;
pub const ERROR_WSMAN_EVENTING_INVALID_NOTIFYTO_ADDRESSS: u32 = 2150858914u32;
pub const ERROR_WSMAN_EVENTING_LOOPBACK_TESTFAILED: u32 = 2150858901u32;
pub const ERROR_WSMAN_EVENTING_MISSING_LOCALE_IN_DELIVERY: u32 = 2150859028u32;
pub const ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO: u32 = 2150858912u32;
pub const ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO_ADDRESSS: u32 = 2150858913u32;
pub const ERROR_WSMAN_EVENTING_NOMATCHING_LISTENER: u32 = 2150858895u32;
pub const ERROR_WSMAN_EVENTING_NONDOMAINJOINED_COLLECTOR: u32 = 2150859070u32;
pub const ERROR_WSMAN_EVENTING_NONDOMAINJOINED_PUBLISHER: u32 = 2150859069u32;
pub const ERROR_WSMAN_EVENTING_PUSH_SUBSCRIPTION_NOACTIVATE_EVENTSOURCE: u32 = 2150859263u32;
pub const ERROR_WSMAN_EVENTING_SOURCE_UNABLE_TO_PROCESS: u32 = 2150858787u32;
pub const ERROR_WSMAN_EVENTING_SUBSCRIPTIONCLOSED_BYREMOTESERVICE: u32 = 2150858907u32;
pub const ERROR_WSMAN_EVENTING_SUBSCRIPTION_CANCELLED_BYSOURCE: u32 = 2150858910u32;
pub const ERROR_WSMAN_EVENTING_UNABLE_TO_RENEW: u32 = 2150858788u32;
pub const ERROR_WSMAN_EVENTING_UNSUPPORTED_EXPIRATION_TYPE: u32 = 2150858784u32;
pub const ERROR_WSMAN_EXPIRATION_TIME_NOT_SUPPORTED: u32 = 2150858856u32;
pub const ERROR_WSMAN_EXPLICIT_CREDENTIALS_REQUIRED: u32 = 2150858981u32;
pub const ERROR_WSMAN_FAILED_AUTHENTICATION: u32 = 2150858806u32;
pub const ERROR_WSMAN_FEATURE_DEPRECATED: u32 = 2150859197u32;
pub const ERROR_WSMAN_FILE_NOT_PRESENT: u32 = 2150859154u32;
pub const ERROR_WSMAN_FILTERING_REQUIRED: u32 = 2150858831u32;
pub const ERROR_WSMAN_FILTERING_REQUIRED_NOT_SUPPORTED: u32 = 2150858864u32;
pub const ERROR_WSMAN_FORMAT_MISMATCH_NOT_SUPPORTED: u32 = 2150858866u32;
pub const ERROR_WSMAN_FORMAT_SECURITY_TOKEN_NOT_SUPPORTED: u32 = 2150858867u32;
pub const ERROR_WSMAN_FRAGMENT_DIALECT_REQUESTED_UNAVAILABLE: u32 = 2150858896u32;
pub const ERROR_WSMAN_FRAGMENT_TRANSFER_NOT_SUPPORTED: u32 = 2150858871u32;
pub const ERROR_WSMAN_GETCLASS: u32 = 2150859245u32;
pub const ERROR_WSMAN_HEARTBEATS_NOT_SUPPORTED: u32 = 2150858858u32;
pub const ERROR_WSMAN_HTML_ERROR: u32 = 2150859123u32;
pub const ERROR_WSMAN_HTTP_CONTENT_TYPE_MISSMATCH_RESPONSE_DATA: u32 = 2150859000u32;
pub const ERROR_WSMAN_HTTP_INVALID_CONTENT_TYPE_IN_RESPONSE_DATA: u32 = 2150858999u32;
pub const ERROR_WSMAN_HTTP_NOT_FOUND_STATUS: u32 = 2150859027u32;
pub const ERROR_WSMAN_HTTP_NO_RESPONSE_DATA: u32 = 2150858997u32;
pub const ERROR_WSMAN_HTTP_REQUEST_TOO_LARGE_STATUS: u32 = 2150859025u32;
pub const ERROR_WSMAN_HTTP_SERVICE_UNAVAILABLE_STATUS: u32 = 2150859026u32;
pub const ERROR_WSMAN_HTTP_STATUS_BAD_REQUEST: u32 = 2150859121u32;
pub const ERROR_WSMAN_HTTP_STATUS_SERVER_ERROR: u32 = 2150859120u32;
pub const ERROR_WSMAN_IISCONFIGURATION_READ_FAILED: u32 = 2150859155u32;
pub const ERROR_WSMAN_INCOMPATIBLE_EPR: u32 = 2150858807u32;
pub const ERROR_WSMAN_INEXISTENT_MAC_ADDRESS: u32 = 2150858875u32;
pub const ERROR_WSMAN_INSECURE_ADDRESS_NOT_SUPPORTED: u32 = 2150858865u32;
pub const ERROR_WSMAN_INSUFFCIENT_SELECTORS: u32 = 2150858842u32;
pub const ERROR_WSMAN_INSUFFICIENT_METADATA_FOR_BASIC: u32 = 2150859251u32;
pub const ERROR_WSMAN_INVALID_ACTIONURI: u32 = 2150858753u32;
pub const ERROR_WSMAN_INVALID_BATCH_PARAMETER: u32 = 2150858799u32;
pub const ERROR_WSMAN_INVALID_BATCH_SETTINGS_PARAMETER: u32 = 2150859021u32;
pub const ERROR_WSMAN_INVALID_BOOKMARK: u32 = 2150858808u32;
pub const ERROR_WSMAN_INVALID_CHARACTERS_IN_RESPONSE: u32 = 2150859018u32;
pub const ERROR_WSMAN_INVALID_CONFIGSDDL_URL: u32 = 2150859199u32;
pub const ERROR_WSMAN_INVALID_CONNECTIONRETRY: u32 = 2150859103u32;
pub const ERROR_WSMAN_INVALID_FILEPATH: u32 = 2150859153u32;
pub const ERROR_WSMAN_INVALID_FILTER_XML: u32 = 2150859015u32;
pub const ERROR_WSMAN_INVALID_FRAGMENT_DIALECT: u32 = 2150858898u32;
pub const ERROR_WSMAN_INVALID_FRAGMENT_PATH: u32 = 2150858899u32;
pub const ERROR_WSMAN_INVALID_FRAGMENT_PATH_BLANK: u32 = 2150859017u32;
pub const ERROR_WSMAN_INVALID_HEADER: u32 = 2150859035u32;
pub const ERROR_WSMAN_INVALID_HOSTNAME_PATTERN: u32 = 2150858911u32;
pub const ERROR_WSMAN_INVALID_IPFILTER: u32 = 2150858988u32;
pub const ERROR_WSMAN_INVALID_KEY: u32 = 2150858820u32;
pub const ERROR_WSMAN_INVALID_LITERAL_URI: u32 = 2150859252u32;
pub const ERROR_WSMAN_INVALID_MESSAGE_INFORMATION_HEADER: u32 = 2150858767u32;
pub const ERROR_WSMAN_INVALID_OPTIONS: u32 = 2150858809u32;
pub const ERROR_WSMAN_INVALID_OPTIONSET: u32 = 2150859140u32;
pub const ERROR_WSMAN_INVALID_OPTION_NO_PROXY_SERVER: u32 = 2150859165u32;
pub const ERROR_WSMAN_INVALID_PARAMETER: u32 = 2150858810u32;
pub const ERROR_WSMAN_INVALID_PARAMETER_NAME: u32 = 2150858837u32;
pub const ERROR_WSMAN_INVALID_PROPOSED_ID: u32 = 2150858798u32;
pub const ERROR_WSMAN_INVALID_PROVIDER_RESPONSE: u32 = 2150859117u32;
pub const ERROR_WSMAN_INVALID_PUBLISHERS_TYPE: u32 = 2150859107u32;
pub const ERROR_WSMAN_INVALID_REDIRECT_ERROR: u32 = 2150859189u32;
pub const ERROR_WSMAN_INVALID_REPRESENTATION: u32 = 2150858773u32;
pub const ERROR_WSMAN_INVALID_RESOURCE_URI: u32 = 2150858811u32;
pub const ERROR_WSMAN_INVALID_RESUMPTION_CONTEXT: u32 = 2150858792u32;
pub const ERROR_WSMAN_INVALID_SECURITY_DESCRIPTOR: u32 = 2150859100u32;
pub const ERROR_WSMAN_INVALID_SELECTORS: u32 = 2150858813u32;
pub const ERROR_WSMAN_INVALID_SELECTOR_NAME: u32 = 2150859032u32;
pub const ERROR_WSMAN_INVALID_SELECTOR_VALUE: u32 = 2150858845u32;
pub const ERROR_WSMAN_INVALID_SOAP_BODY: u32 = 2150858791u32;
pub const ERROR_WSMAN_INVALID_SUBSCRIBE_OBJECT: u32 = 2150859112u32;
pub const ERROR_WSMAN_INVALID_SUBSCRIPTION_MANAGER: u32 = 2150859006u32;
pub const ERROR_WSMAN_INVALID_SYSTEM: u32 = 2150858812u32;
pub const ERROR_WSMAN_INVALID_TARGET_RESOURCEURI: u32 = 2150858849u32;
pub const ERROR_WSMAN_INVALID_TARGET_SELECTORS: u32 = 2150858848u32;
pub const ERROR_WSMAN_INVALID_TARGET_SYSTEM: u32 = 2150858850u32;
pub const ERROR_WSMAN_INVALID_TIMEOUT_HEADER: u32 = 2150858881u32;
pub const ERROR_WSMAN_INVALID_URI: u32 = 2150858754u32;
pub const ERROR_WSMAN_INVALID_URI_WMI_ENUM_WQL: u32 = 2150859003u32;
pub const ERROR_WSMAN_INVALID_URI_WMI_SINGLETON: u32 = 2150859002u32;
pub const ERROR_WSMAN_INVALID_USESSL_PARAM: u32 = 2150859198u32;
pub const ERROR_WSMAN_INVALID_XML: u32 = 2150858819u32;
pub const ERROR_WSMAN_INVALID_XML_FRAGMENT: u32 = 2150858841u32;
pub const ERROR_WSMAN_INVALID_XML_MISSING_VALUES: u32 = 2150858839u32;
pub const ERROR_WSMAN_INVALID_XML_NAMESPACE: u32 = 2150858840u32;
pub const ERROR_WSMAN_INVALID_XML_RUNAS_DISABLED: u32 = 2150859232u32;
pub const ERROR_WSMAN_INVALID_XML_VALUES: u32 = 2150858838u32;
pub const ERROR_WSMAN_KERBEROS_IPADDRESS: u32 = 2150859019u32;
pub const ERROR_WSMAN_LISTENER_ADDRESS_INVALID: u32 = 2150858889u32;
pub const ERROR_WSMAN_LOCALE_NOT_SUPPORTED: u32 = 2150858855u32;
pub const ERROR_WSMAN_MACHINE_OPTION_REQUIRED: u32 = 2150858917u32;
pub const ERROR_WSMAN_MAXENVELOPE_POLICY_NOT_SUPPORTED: u32 = 2150858863u32;
pub const ERROR_WSMAN_MAXENVELOPE_SIZE_NOT_SUPPORTED: u32 = 2150858862u32;
pub const ERROR_WSMAN_MAXITEMS_NOT_SUPPORTED: u32 = 2150858860u32;
pub const ERROR_WSMAN_MAXTIME_NOT_SUPPORTED: u32 = 2150858861u32;
pub const ERROR_WSMAN_MAX_ELEMENTS_NOT_SUPPORTED: u32 = 2150859037u32;
pub const ERROR_WSMAN_MAX_ENVELOPE_SIZE: u32 = 2150858823u32;
pub const ERROR_WSMAN_MAX_ENVELOPE_SIZE_EXCEEDED: u32 = 2150858824u32;
pub const ERROR_WSMAN_MESSAGE_INFORMATION_HEADER_REQUIRED: u32 = 2150858769u32;
pub const ERROR_WSMAN_METADATA_REDIRECT: u32 = 2150858814u32;
pub const ERROR_WSMAN_MIN_ENVELOPE_SIZE: u32 = 2150858878u32;
pub const ERROR_WSMAN_MISSING_CLASSNAME: u32 = 2150859254u32;
pub const ERROR_WSMAN_MISSING_FRAGMENT_PATH: u32 = 2150858897u32;
pub const ERROR_WSMAN_MULTIPLE_CREDENTIALS: u32 = 2150859076u32;
pub const ERROR_WSMAN_MUSTUNDERSTAND_ON_LOCALE_UNSUPPORTED: u32 = 2150858887u32;
pub const ERROR_WSMAN_MUTUAL_AUTH_FAILED: u32 = 2150859248u32;
pub const ERROR_WSMAN_NAME_NOT_RESOLVED: u32 = 2150859193u32;
pub const ERROR_WSMAN_NETWORK_TIMEDOUT: u32 = 2150859046u32;
pub const ERROR_WSMAN_NEW_DESERIALIZER: u32 = 2150859243u32;
pub const ERROR_WSMAN_NEW_SESSION: u32 = 2150859246u32;
pub const ERROR_WSMAN_NON_PULL_SUBSCRIPTION_NOT_SUPPORTED: u32 = 2150859007u32;
pub const ERROR_WSMAN_NO_ACK: u32 = 2150858800u32;
pub const ERROR_WSMAN_NO_CERTMAPPING_OPERATION_FOR_LOCAL_SESSION: u32 = 2150859090u32;
pub const ERROR_WSMAN_NO_COMMANDID: u32 = 2150859141u32;
pub const ERROR_WSMAN_NO_COMMAND_RESPONSE: u32 = 2150859139u32;
pub const ERROR_WSMAN_NO_DHCP_ADDRESSES: u32 = 2150858877u32;
pub const ERROR_WSMAN_NO_IDENTIFY_FOR_LOCAL_SESSION: u32 = 2150859004u32;
pub const ERROR_WSMAN_NO_PUSH_SUBSCRIPTION_FOR_LOCAL_SESSION: u32 = 2150859005u32;
pub const ERROR_WSMAN_NO_RECEIVE_RESPONSE: u32 = 2150859151u32;
pub const ERROR_WSMAN_NO_UNICAST_ADDRESSES: u32 = 2150858876u32;
pub const ERROR_WSMAN_NULL_KEY: u32 = 2150859247u32;
pub const ERROR_WSMAN_OBJECTONLY_INVALID: u32 = 2150859253u32;
pub const ERROR_WSMAN_OPERATION_TIMEDOUT: u32 = 2150858793u32;
pub const ERROR_WSMAN_OPERATION_TIMEOUT_NOT_SUPPORTED: u32 = 2150858854u32;
pub const ERROR_WSMAN_OPTIONS_INVALID_NAME: u32 = 2150858834u32;
pub const ERROR_WSMAN_OPTIONS_INVALID_VALUE: u32 = 2150858835u32;
pub const ERROR_WSMAN_OPTIONS_NOT_SUPPORTED: u32 = 2150858833u32;
pub const ERROR_WSMAN_OPTION_LIMIT: u32 = 2150858827u32;
pub const ERROR_WSMAN_PARAMETER_TYPE_MISMATCH: u32 = 2150858836u32;
pub const ERROR_WSMAN_PLUGIN_CONFIGURATION_CORRUPTED: u32 = 2150859152u32;
pub const ERROR_WSMAN_PLUGIN_FAILED: u32 = 2150858883u32;
pub const ERROR_WSMAN_POLICY_CANNOT_COMPLY: u32 = 2150859102u32;
pub const ERROR_WSMAN_POLICY_CORRUPTED: u32 = 2150858888u32;
pub const ERROR_WSMAN_POLICY_TOO_COMPLEX: u32 = 2150859101u32;
pub const ERROR_WSMAN_POLYMORPHISM_MODE_UNSUPPORTED: u32 = 2150859063u32;
pub const ERROR_WSMAN_PORT_INVALID: u32 = 2150858971u32;
pub const ERROR_WSMAN_PROVIDER_FAILURE: u32 = 2150858755u32;
pub const ERROR_WSMAN_PROVIDER_LOAD_FAILED: u32 = 2150858906u32;
pub const ERROR_WSMAN_PROVSYS_NOT_SUPPORTED: u32 = 2150858921u32;
pub const ERROR_WSMAN_PROXY_ACCESS_TYPE: u32 = 2150859164u32;
pub const ERROR_WSMAN_PROXY_AUTHENTICATION_INVALID_FLAG: u32 = 2150859162u32;
pub const ERROR_WSMAN_PUBLIC_FIREWALL_PROFILE_ACTIVE: u32 = 2150859113u32;
pub const ERROR_WSMAN_PULL_IN_PROGRESS: u32 = 2150858758u32;
pub const ERROR_WSMAN_PULL_PARAMS_NOT_SAME_AS_ENUM: u32 = 2150859181u32;
pub const ERROR_WSMAN_PUSHSUBSCRIPTION_INVALIDUSERACCOUNT: u32 = 2150859068u32;
pub const ERROR_WSMAN_PUSH_SUBSCRIPTION_CONFIG_INVALID: u32 = 2150858922u32;
pub const ERROR_WSMAN_QUICK_CONFIG_FAILED_CERT_REQUIRED: u32 = 2150859029u32;
pub const ERROR_WSMAN_QUICK_CONFIG_FIREWALL_EXCEPTIONS_DISALLOWED: u32 = 2150859030u32;
pub const ERROR_WSMAN_QUICK_CONFIG_LOCAL_POLICY_CHANGE_DISALLOWED: u32 = 2150859031u32;
pub const ERROR_WSMAN_QUOTA_LIMIT: u32 = 2150858815u32;
pub const ERROR_WSMAN_QUOTA_MAX_COMMANDS_PER_SHELL_PPQ: u32 = 2150859241u32;
pub const ERROR_WSMAN_QUOTA_MAX_OPERATIONS: u32 = 2150859174u32;
pub const ERROR_WSMAN_QUOTA_MAX_OPERATIONS_USER_PPQ: u32 = 2150859240u32;
pub const ERROR_WSMAN_QUOTA_MAX_PLUGINOPERATIONS_PPQ: u32 = 2150859239u32;
pub const ERROR_WSMAN_QUOTA_MAX_PLUGINSHELLS_PPQ: u32 = 2150859238u32;
pub const ERROR_WSMAN_QUOTA_MAX_SHELLS: u32 = 2150859173u32;
pub const ERROR_WSMAN_QUOTA_MAX_SHELLS_PPQ: u32 = 2150859236u32;
pub const ERROR_WSMAN_QUOTA_MAX_SHELLUSERS: u32 = 2150859179u32;
pub const ERROR_WSMAN_QUOTA_MAX_USERS_PPQ: u32 = 2150859237u32;
pub const ERROR_WSMAN_QUOTA_MIN_REQUIREMENT_NOT_AVAILABLE_PPQ: u32 = 2150859242u32;
pub const ERROR_WSMAN_QUOTA_SYSTEM: u32 = 2150859176u32;
pub const ERROR_WSMAN_QUOTA_USER: u32 = 2150859175u32;
pub const ERROR_WSMAN_REDIRECT_LOCATION_NOT_AVAILABLE: u32 = 2150859178u32;
pub const ERROR_WSMAN_REDIRECT_REQUESTED: u32 = 2150859161u32;
pub const ERROR_WSMAN_REMOTESHELLS_NOT_ALLOWED: u32 = 2150859180u32;
pub const ERROR_WSMAN_REMOTE_CIMPATH_NOT_SUPPORTED: u32 = 2150859009u32;
pub const ERROR_WSMAN_REMOTE_CONNECTION_NOT_ALLOWED: u32 = 2150859235u32;
pub const ERROR_WSMAN_RENAME_FAILURE: u32 = 2150858816u32;
pub const ERROR_WSMAN_REQUEST_INIT_ERROR: u32 = 2150858880u32;
pub const ERROR_WSMAN_REQUEST_NOT_SUPPORTED_AT_SERVICE: u32 = 2150859064u32;
pub const ERROR_WSMAN_RESOURCE_NOT_FOUND: u32 = 2150858752u32;
pub const ERROR_WSMAN_RESPONSE_INVALID_ENUMERATION_CONTEXT: u32 = 2150858993u32;
pub const ERROR_WSMAN_RESPONSE_INVALID_MESSAGE_INFORMATION_HEADER: u32 = 2150858995u32;
pub const ERROR_WSMAN_RESPONSE_INVALID_SOAP_FAULT: u32 = 2150858998u32;
pub const ERROR_WSMAN_RESPONSE_NO_RESULTS: u32 = 2150858991u32;
pub const ERROR_WSMAN_RESPONSE_NO_SOAP_HEADER_BODY: u32 = 2150858996u32;
pub const ERROR_WSMAN_RESPONSE_NO_XML_FRAGMENT_WRAPPER: u32 = 2150858994u32;
pub const ERROR_WSMAN_RESUMPTION_NOT_SUPPORTED: u32 = 2150858794u32;
pub const ERROR_WSMAN_RESUMPTION_TYPE_NOT_SUPPORTED: u32 = 2150858795u32;
pub const ERROR_WSMAN_RUNASUSER_MANAGEDACCOUNT_LOGON_FAILED: u32 = 2150859261u32;
pub const ERROR_WSMAN_RUNAS_INVALIDUSERCREDENTIALS: u32 = 2150859203u32;
pub const ERROR_WSMAN_RUNSHELLCOMMAND_NULL_ARGUMENT: u32 = 2150859086u32;
pub const ERROR_WSMAN_SCHEMA_VALIDATION_ERROR: u32 = 2150858817u32;
pub const ERROR_WSMAN_SECURITY_UNMAPPED: u32 = 2150858909u32;
pub const ERROR_WSMAN_SELECTOR_LIMIT: u32 = 2150858826u32;
pub const ERROR_WSMAN_SELECTOR_TYPEMISMATCH: u32 = 2150858844u32;
pub const ERROR_WSMAN_SEMANTICCALLBACK_TIMEDOUT: u32 = 2150859228u32;
pub const ERROR_WSMAN_SENDHEARBEAT_EMPTY_ENUMERATOR: u32 = 2150858973u32;
pub const ERROR_WSMAN_SENDSHELLINPUT_INVALID_STREAMID_INDEX: u32 = 2150859088u32;
pub const ERROR_WSMAN_SERVER_DESTINATION_LOCALHOST: u32 = 2150859022u32;
pub const ERROR_WSMAN_SERVER_ENVELOPE_LIMIT: u32 = 2150858825u32;
pub const ERROR_WSMAN_SERVER_NONPULLSUBSCRIBE_NULL_PARAM: u32 = 2150858966u32;
pub const ERROR_WSMAN_SERVER_NOT_TRUSTED: u32 = 2150858980u32;
pub const ERROR_WSMAN_SERVICE_REMOTE_ACCESS_DISABLED: u32 = 2150859229u32;
pub const ERROR_WSMAN_SERVICE_STREAM_DISCONNECTED: u32 = 2150859230u32;
pub const ERROR_WSMAN_SESSION_ALREADY_CLOSED: u32 = 2150858904u32;
pub const ERROR_WSMAN_SHELL_ALREADY_CLOSED: u32 = 2150859082u32;
pub const ERROR_WSMAN_SHELL_INVALID_COMMAND_HANDLE: u32 = 2150859085u32;
pub const ERROR_WSMAN_SHELL_INVALID_DESIRED_STREAMS: u32 = 2150859149u32;
pub const ERROR_WSMAN_SHELL_INVALID_INPUT_STREAM: u32 = 2150859147u32;
pub const ERROR_WSMAN_SHELL_INVALID_SHELL_HANDLE: u32 = 2150859084u32;
pub const ERROR_WSMAN_SHELL_NOT_INITIALIZED: u32 = 2150859118u32;
pub const ERROR_WSMAN_SHELL_SYNCHRONOUS_NOT_SUPPORTED: u32 = 2150859089u32;
pub const ERROR_WSMAN_SOAP_DATA_ENCODING_UNKNOWN: u32 = 2150858766u32;
pub const ERROR_WSMAN_SOAP_FAULT_MUST_UNDERSTAND: u32 = 2150858768u32;
pub const ERROR_WSMAN_SOAP_VERSION_MISMATCH: u32 = 2150858765u32;
pub const ERROR_WSMAN_SSL_CONNECTION_ABORTED: u32 = 2150859194u32;
pub const ERROR_WSMAN_SUBSCRIBE_WMI_INVALID_KEY: u32 = 2150859225u32;
pub const ERROR_WSMAN_SUBSCRIPTION_CLIENT_DID_NOT_CALL_WITHIN_HEARTBEAT: u32 = 2150858762u32;
pub const ERROR_WSMAN_SUBSCRIPTION_CLOSED: u32 = 2150858760u32;
pub const ERROR_WSMAN_SUBSCRIPTION_CLOSE_IN_PROGRESS: u32 = 2150858761u32;
pub const ERROR_WSMAN_SUBSCRIPTION_LISTENER_NOLONGERVALID: u32 = 2150858905u32;
pub const ERROR_WSMAN_SUBSCRIPTION_NO_HEARTBEAT: u32 = 2150858763u32;
pub const ERROR_WSMAN_SYSTEM_NOT_FOUND: u32 = 2150858822u32;
pub const ERROR_WSMAN_TARGET_ALREADY_EXISTS: u32 = 2150858851u32;
pub const ERROR_WSMAN_TRANSPORT_NOT_SUPPORTED: u32 = 2150858970u32;
pub const ERROR_WSMAN_UNEXPECTED_SELECTORS: u32 = 2150858843u32;
pub const ERROR_WSMAN_UNKNOWN_HTTP_STATUS_RETURNED: u32 = 2150859023u32;
pub const ERROR_WSMAN_UNREPORTABLE_SUCCESS: u32 = 2150858829u32;
pub const ERROR_WSMAN_UNSUPPORTED_ADDRESSING_MODE: u32 = 2150858870u32;
pub const ERROR_WSMAN_UNSUPPORTED_ENCODING: u32 = 2150858796u32;
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE: u32 = 2150858818u32;
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE_IDENTIFY: u32 = 2150859257u32;
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE_OPTIONS: u32 = 2150858918u32;
pub const ERROR_WSMAN_UNSUPPORTED_HTTP_STATUS_REDIRECT: u32 = 2150859024u32;
pub const ERROR_WSMAN_UNSUPPORTED_MEDIA: u32 = 2150858869u32;
pub const ERROR_WSMAN_UNSUPPORTED_OCTETTYPE: u32 = 2150859249u32;
pub const ERROR_WSMAN_UNSUPPORTED_TIMEOUT: u32 = 2150858764u32;
pub const ERROR_WSMAN_UNSUPPORTED_TYPE: u32 = 2150859234u32;
pub const ERROR_WSMAN_URISECURITY_INVALIDURIKEY: u32 = 2150859104u32;
pub const ERROR_WSMAN_URI_LIMIT: u32 = 2150858797u32;
pub const ERROR_WSMAN_URI_NON_DMTF_CLASS: u32 = 2150859065u32;
pub const ERROR_WSMAN_URI_QUERY_STRING_SYNTAX_ERROR: u32 = 2150858874u32;
pub const ERROR_WSMAN_URI_SECURITY_URI: u32 = 2150859183u32;
pub const ERROR_WSMAN_URI_WRONG_DMTF_VERSION: u32 = 2150859066u32;
pub const ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED: u32 = 2150859259u32;
pub const ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED_DOWNLEVEL: u32 = 2150859260u32;
pub const ERROR_WSMAN_WHITESPACE: u32 = 2150858830u32;
pub const ERROR_WSMAN_WMI_CANNOT_CONNECT_ACCESS_DENIED: u32 = 2150859014u32;
pub const ERROR_WSMAN_WMI_INVALID_VALUE: u32 = 2150859011u32;
pub const ERROR_WSMAN_WMI_MAX_NESTED: u32 = 2150859008u32;
pub const ERROR_WSMAN_WMI_PROVIDER_ACCESS_DENIED: u32 = 2150859013u32;
pub const ERROR_WSMAN_WMI_PROVIDER_INVALID_PARAMETER: u32 = 2150859038u32;
pub const ERROR_WSMAN_WMI_PROVIDER_NOT_CAPABLE: u32 = 2150859010u32;
pub const ERROR_WSMAN_WMI_SVC_ACCESS_DENIED: u32 = 2150859012u32;
pub const ERROR_WSMAN_WRONG_METADATA: u32 = 2150859233u32;
pub const WSMAN_CMDSHELL_OPTION_CODEPAGE: ::windows_core::PCWSTR = ::windows_core::w!("WINRS_CODEPAGE");
pub const WSMAN_CMDSHELL_OPTION_CONSOLEMODE_STDIN: ::windows_core::PCWSTR = ::windows_core::w!("WINRS_CONSOLEMODE_STDIN");
pub const WSMAN_CMDSHELL_OPTION_SKIP_CMD_SHELL: ::windows_core::PCWSTR = ::windows_core::w!("WINRS_SKIP_CMD_SHELL");
pub const WSMAN_DATA_NONE: WSManDataType = WSManDataType(0i32);
pub const WSMAN_DATA_TYPE_BINARY: WSManDataType = WSManDataType(2i32);
pub const WSMAN_DATA_TYPE_DWORD: WSManDataType = WSManDataType(4i32);
pub const WSMAN_DATA_TYPE_TEXT: WSManDataType = WSManDataType(1i32);
pub const WSMAN_DEFAULT_TIMEOUT_MS: u32 = 60000u32;
pub const WSMAN_FLAG_AUTH_BASIC: WSManAuthenticationFlags = WSManAuthenticationFlags(8i32);
pub const WSMAN_FLAG_AUTH_CLIENT_CERTIFICATE: WSManAuthenticationFlags = WSManAuthenticationFlags(32i32);
pub const WSMAN_FLAG_AUTH_CREDSSP: WSManAuthenticationFlags = WSManAuthenticationFlags(128i32);
pub const WSMAN_FLAG_AUTH_DIGEST: WSManAuthenticationFlags = WSManAuthenticationFlags(2i32);
pub const WSMAN_FLAG_AUTH_KERBEROS: WSManAuthenticationFlags = WSManAuthenticationFlags(16i32);
pub const WSMAN_FLAG_AUTH_NEGOTIATE: WSManAuthenticationFlags = WSManAuthenticationFlags(4i32);
pub const WSMAN_FLAG_CALLBACK_END_OF_OPERATION: WSManCallbackFlags = WSManCallbackFlags(1i32);
pub const WSMAN_FLAG_CALLBACK_END_OF_STREAM: WSManCallbackFlags = WSManCallbackFlags(8i32);
pub const WSMAN_FLAG_CALLBACK_NETWORK_FAILURE_DETECTED: WSManCallbackFlags = WSManCallbackFlags(256i32);
pub const WSMAN_FLAG_CALLBACK_RECEIVE_DELAY_STREAM_REQUEST_PROCESSED: WSManCallbackFlags = WSManCallbackFlags(8192i32);
pub const WSMAN_FLAG_CALLBACK_RECONNECTED_AFTER_NETWORK_FAILURE: WSManCallbackFlags = WSManCallbackFlags(1024i32);
pub const WSMAN_FLAG_CALLBACK_RETRYING_AFTER_NETWORK_FAILURE: WSManCallbackFlags = WSManCallbackFlags(512i32);
pub const WSMAN_FLAG_CALLBACK_RETRY_ABORTED_DUE_TO_INTERNAL_ERROR: WSManCallbackFlags = WSManCallbackFlags(4096i32);
pub const WSMAN_FLAG_CALLBACK_SHELL_AUTODISCONNECTED: WSManCallbackFlags = WSManCallbackFlags(64i32);
pub const WSMAN_FLAG_CALLBACK_SHELL_AUTODISCONNECTING: WSManCallbackFlags = WSManCallbackFlags(2048i32);
pub const WSMAN_FLAG_CALLBACK_SHELL_SUPPORTS_DISCONNECT: WSManCallbackFlags = WSManCallbackFlags(32i32);
pub const WSMAN_FLAG_DEFAULT_AUTHENTICATION: WSManAuthenticationFlags = WSManAuthenticationFlags(0i32);
pub const WSMAN_FLAG_DELETE_SERVER_SESSION: WSManShellFlag = WSManShellFlag(2i32);
pub const WSMAN_FLAG_NO_AUTHENTICATION: WSManAuthenticationFlags = WSManAuthenticationFlags(1i32);
pub const WSMAN_FLAG_NO_COMPRESSION: WSManShellFlag = WSManShellFlag(1i32);
pub const WSMAN_FLAG_RECEIVE_DELAY_OUTPUT_STREAM: WSManShellFlag = WSManShellFlag(16i32);
pub const WSMAN_FLAG_RECEIVE_FLUSH: u32 = 2u32;
pub const WSMAN_FLAG_RECEIVE_RESULT_DATA_BOUNDARY: u32 = 4u32;
pub const WSMAN_FLAG_RECEIVE_RESULT_NO_MORE_DATA: u32 = 1u32;
pub const WSMAN_FLAG_REQUESTED_API_VERSION_1_0: u32 = 0u32;
pub const WSMAN_FLAG_REQUESTED_API_VERSION_1_1: u32 = 1u32;
pub const WSMAN_FLAG_SEND_NO_MORE_DATA: u32 = 1u32;
pub const WSMAN_FLAG_SERVER_BUFFERING_MODE_BLOCK: WSManShellFlag = WSManShellFlag(8i32);
pub const WSMAN_FLAG_SERVER_BUFFERING_MODE_DROP: WSManShellFlag = WSManShellFlag(4i32);
pub const WSMAN_OPERATION_INFOV1: u32 = 0u32;
pub const WSMAN_OPERATION_INFOV2: u32 = 2864434397u32;
pub const WSMAN_OPTION_ALLOW_NEGOTIATE_IMPLICIT_CREDENTIALS: WSManSessionOption = WSManSessionOption(32i32);
pub const WSMAN_OPTION_DEFAULT_OPERATION_TIMEOUTMS: WSManSessionOption = WSManSessionOption(1i32);
pub const WSMAN_OPTION_ENABLE_SPN_SERVER_PORT: WSManSessionOption = WSManSessionOption(22i32);
pub const WSMAN_OPTION_LOCALE: WSManSessionOption = WSManSessionOption(25i32);
pub const WSMAN_OPTION_MACHINE_ID: WSManSessionOption = WSManSessionOption(23i32);
pub const WSMAN_OPTION_MAX_ENVELOPE_SIZE_KB: WSManSessionOption = WSManSessionOption(28i32);
pub const WSMAN_OPTION_MAX_RETRY_TIME: WSManSessionOption = WSManSessionOption(11i32);
pub const WSMAN_OPTION_PROXY_AUTO_DETECT: WSManProxyAccessType = WSManProxyAccessType(4i32);
pub const WSMAN_OPTION_PROXY_IE_PROXY_CONFIG: WSManProxyAccessType = WSManProxyAccessType(1i32);
pub const WSMAN_OPTION_PROXY_NO_PROXY_SERVER: WSManProxyAccessType = WSManProxyAccessType(8i32);
pub const WSMAN_OPTION_PROXY_WINHTTP_PROXY_CONFIG: WSManProxyAccessType = WSManProxyAccessType(2i32);
pub const WSMAN_OPTION_REDIRECT_LOCATION: WSManSessionOption = WSManSessionOption(30i32);
pub const WSMAN_OPTION_SHELL_MAX_DATA_SIZE_PER_MESSAGE_KB: WSManSessionOption = WSManSessionOption(29i32);
pub const WSMAN_OPTION_SKIP_CA_CHECK: WSManSessionOption = WSManSessionOption(18i32);
pub const WSMAN_OPTION_SKIP_CN_CHECK: WSManSessionOption = WSManSessionOption(19i32);
pub const WSMAN_OPTION_SKIP_REVOCATION_CHECK: WSManSessionOption = WSManSessionOption(31i32);
pub const WSMAN_OPTION_TIMEOUTMS_CLOSE_SHELL: WSManSessionOption = WSManSessionOption(17i32);
pub const WSMAN_OPTION_TIMEOUTMS_CREATE_SHELL: WSManSessionOption = WSManSessionOption(12i32);
pub const WSMAN_OPTION_TIMEOUTMS_RECEIVE_SHELL_OUTPUT: WSManSessionOption = WSManSessionOption(14i32);
pub const WSMAN_OPTION_TIMEOUTMS_RUN_SHELL_COMMAND: WSManSessionOption = WSManSessionOption(13i32);
pub const WSMAN_OPTION_TIMEOUTMS_SEND_SHELL_INPUT: WSManSessionOption = WSManSessionOption(15i32);
pub const WSMAN_OPTION_TIMEOUTMS_SIGNAL_SHELL: WSManSessionOption = WSManSessionOption(16i32);
pub const WSMAN_OPTION_UI_LANGUAGE: WSManSessionOption = WSManSessionOption(26i32);
pub const WSMAN_OPTION_UNENCRYPTED_MESSAGES: WSManSessionOption = WSManSessionOption(20i32);
pub const WSMAN_OPTION_USE_INTEARACTIVE_TOKEN: WSManSessionOption = WSManSessionOption(34i32);
pub const WSMAN_OPTION_USE_SSL: WSManSessionOption = WSManSessionOption(33i32);
pub const WSMAN_OPTION_UTF16: WSManSessionOption = WSManSessionOption(21i32);
pub const WSMAN_PLUGIN_PARAMS_AUTORESTART: u32 = 3u32;
pub const WSMAN_PLUGIN_PARAMS_GET_REQUESTED_DATA_LOCALE: u32 = 6u32;
pub const WSMAN_PLUGIN_PARAMS_GET_REQUESTED_LOCALE: u32 = 5u32;
pub const WSMAN_PLUGIN_PARAMS_HOSTIDLETIMEOUTSECONDS: u32 = 4u32;
pub const WSMAN_PLUGIN_PARAMS_LARGEST_RESULT_SIZE: u32 = 4u32;
pub const WSMAN_PLUGIN_PARAMS_MAX_ENVELOPE_SIZE: u32 = 1u32;
pub const WSMAN_PLUGIN_PARAMS_NAME: u32 = 5u32;
pub const WSMAN_PLUGIN_PARAMS_REMAINING_RESULT_SIZE: u32 = 3u32;
pub const WSMAN_PLUGIN_PARAMS_RUNAS_USER: u32 = 2u32;
pub const WSMAN_PLUGIN_PARAMS_SHAREDHOST: u32 = 1u32;
pub const WSMAN_PLUGIN_PARAMS_TIMEOUT: u32 = 2u32;
pub const WSMAN_PLUGIN_SHUTDOWN_IDLETIMEOUT_ELAPSED: u32 = 4u32;
pub const WSMAN_PLUGIN_SHUTDOWN_IISHOST: u32 = 3u32;
pub const WSMAN_PLUGIN_SHUTDOWN_SERVICE: u32 = 2u32;
pub const WSMAN_PLUGIN_SHUTDOWN_SYSTEM: u32 = 1u32;
pub const WSMAN_PLUGIN_STARTUP_AUTORESTARTED_CRASH: u32 = 2u32;
pub const WSMAN_PLUGIN_STARTUP_AUTORESTARTED_REBOOT: u32 = 1u32;
pub const WSMAN_PLUGIN_STARTUP_REQUEST_RECEIVED: u32 = 0u32;
pub const WSMAN_SHELL_NS: ::windows_core::PCWSTR = ::windows_core::w!("http://schemas.microsoft.com/wbem/wsman/1/windows/shell");
pub const WSMAN_SHELL_OPTION_NOPROFILE: ::windows_core::PCWSTR = ::windows_core::w!("WINRS_NOPROFILE");
pub const WSMAN_STREAM_ID_STDERR: ::windows_core::PCWSTR = ::windows_core::w!("stderr");
pub const WSMAN_STREAM_ID_STDIN: ::windows_core::PCWSTR = ::windows_core::w!("stdin");
pub const WSMAN_STREAM_ID_STDOUT: ::windows_core::PCWSTR = ::windows_core::w!("stdout");
pub const WSMan: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbced617b_ec03_420b_8508_977dc7a686bd);
pub const WSManFlagAllowNegotiateImplicitCredentials: WSManSessionFlags = WSManSessionFlags(67108864i32);
pub const WSManFlagAssociatedInstance: WSManEnumFlags = WSManEnumFlags(0i32);
pub const WSManFlagAssociationInstance: WSManEnumFlags = WSManEnumFlags(128i32);
pub const WSManFlagCredUsernamePassword: WSManSessionFlags = WSManSessionFlags(4096i32);
pub const WSManFlagEnableSPNServerPort: WSManSessionFlags = WSManSessionFlags(4194304i32);
pub const WSManFlagHierarchyDeep: WSManEnumFlags = WSManEnumFlags(0i32);
pub const WSManFlagHierarchyDeepBasePropsOnly: WSManEnumFlags = WSManEnumFlags(64i32);
pub const WSManFlagHierarchyShallow: WSManEnumFlags = WSManEnumFlags(32i32);
pub const WSManFlagNoEncryption: WSManSessionFlags = WSManSessionFlags(1048576i32);
pub const WSManFlagNonXmlText: WSManEnumFlags = WSManEnumFlags(1i32);
pub const WSManFlagProxyAuthenticationUseBasic: WSManProxyAuthenticationFlags = WSManProxyAuthenticationFlags(2i32);
pub const WSManFlagProxyAuthenticationUseDigest: WSManProxyAuthenticationFlags = WSManProxyAuthenticationFlags(4i32);
pub const WSManFlagProxyAuthenticationUseNegotiate: WSManProxyAuthenticationFlags = WSManProxyAuthenticationFlags(1i32);
pub const WSManFlagReturnEPR: WSManEnumFlags = WSManEnumFlags(2i32);
pub const WSManFlagReturnObject: WSManEnumFlags = WSManEnumFlags(0i32);
pub const WSManFlagReturnObjectAndEPR: WSManEnumFlags = WSManEnumFlags(4i32);
pub const WSManFlagSkipCACheck: WSManSessionFlags = WSManSessionFlags(8192i32);
pub const WSManFlagSkipCNCheck: WSManSessionFlags = WSManSessionFlags(16384i32);
pub const WSManFlagSkipRevocationCheck: WSManSessionFlags = WSManSessionFlags(33554432i32);
pub const WSManFlagUTF16: WSManSessionFlags = WSManSessionFlags(8388608i32);
pub const WSManFlagUTF8: WSManSessionFlags = WSManSessionFlags(1i32);
pub const WSManFlagUseBasic: WSManSessionFlags = WSManSessionFlags(262144i32);
pub const WSManFlagUseClientCertificate: WSManSessionFlags = WSManSessionFlags(2097152i32);
pub const WSManFlagUseCredSsp: WSManSessionFlags = WSManSessionFlags(16777216i32);
pub const WSManFlagUseDigest: WSManSessionFlags = WSManSessionFlags(65536i32);
pub const WSManFlagUseKerberos: WSManSessionFlags = WSManSessionFlags(524288i32);
pub const WSManFlagUseNegotiate: WSManSessionFlags = WSManSessionFlags(131072i32);
pub const WSManFlagUseNoAuthentication: WSManSessionFlags = WSManSessionFlags(32768i32);
pub const WSManFlagUseSsl: WSManSessionFlags = WSManSessionFlags(134217728i32);
pub const WSManInternal: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7de087a5_5dcb_4df7_bb12_0924ad8fbd9a);
pub const WSManProxyAutoDetect: WSManProxyAccessTypeFlags = WSManProxyAccessTypeFlags(4i32);
pub const WSManProxyIEConfig: WSManProxyAccessTypeFlags = WSManProxyAccessTypeFlags(1i32);
pub const WSManProxyNoProxyServer: WSManProxyAccessTypeFlags = WSManProxyAccessTypeFlags(8i32);
pub const WSManProxyWinHttpConfig: WSManProxyAccessTypeFlags = WSManProxyAccessTypeFlags(2i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManAuthenticationFlags(pub i32);
impl ::windows_core::TypeKind for WSManAuthenticationFlags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManAuthenticationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManAuthenticationFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManCallbackFlags(pub i32);
impl ::windows_core::TypeKind for WSManCallbackFlags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManCallbackFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManCallbackFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManDataType(pub i32);
impl ::windows_core::TypeKind for WSManDataType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManDataType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManEnumFlags(pub i32);
impl ::windows_core::TypeKind for WSManEnumFlags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManEnumFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManEnumFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManProxyAccessType(pub i32);
impl ::windows_core::TypeKind for WSManProxyAccessType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManProxyAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManProxyAccessType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManProxyAccessTypeFlags(pub i32);
impl ::windows_core::TypeKind for WSManProxyAccessTypeFlags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManProxyAccessTypeFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManProxyAccessTypeFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManProxyAuthenticationFlags(pub i32);
impl ::windows_core::TypeKind for WSManProxyAuthenticationFlags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManProxyAuthenticationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManProxyAuthenticationFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManSessionFlags(pub i32);
impl ::windows_core::TypeKind for WSManSessionFlags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManSessionFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManSessionFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManSessionOption(pub i32);
impl ::windows_core::TypeKind for WSManSessionOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManSessionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManSessionOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WSManShellFlag(pub i32);
impl ::windows_core::TypeKind for WSManShellFlag {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WSManShellFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManShellFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSMAN_API_HANDLE(pub isize);
impl ::core::default::Default for WSMAN_API_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for WSMAN_API_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for WSMAN_API_HANDLE {}
impl ::core::fmt::Debug for WSMAN_API_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSMAN_API_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_API_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct WSMAN_AUTHENTICATION_CREDENTIALS {
    pub authenticationMechanism: u32,
    pub Anonymous: WSMAN_AUTHENTICATION_CREDENTIALS_0,
}
impl ::core::marker::Copy for WSMAN_AUTHENTICATION_CREDENTIALS {}
impl ::core::clone::Clone for WSMAN_AUTHENTICATION_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_AUTHENTICATION_CREDENTIALS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_AUTHENTICATION_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WSMAN_AUTHENTICATION_CREDENTIALS_0 {
    pub userAccount: WSMAN_USERNAME_PASSWORD_CREDS,
    pub certificateThumbprint: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_AUTHENTICATION_CREDENTIALS_0 {}
impl ::core::clone::Clone for WSMAN_AUTHENTICATION_CREDENTIALS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_AUTHENTICATION_CREDENTIALS_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_AUTHENTICATION_CREDENTIALS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_AUTHZ_QUOTA {
    pub maxAllowedConcurrentShells: u32,
    pub maxAllowedConcurrentOperations: u32,
    pub timeslotSize: u32,
    pub maxAllowedOperationsPerTimeslot: u32,
}
impl ::core::marker::Copy for WSMAN_AUTHZ_QUOTA {}
impl ::core::clone::Clone for WSMAN_AUTHZ_QUOTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_AUTHZ_QUOTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_AUTHZ_QUOTA").field("maxAllowedConcurrentShells", &self.maxAllowedConcurrentShells).field("maxAllowedConcurrentOperations", &self.maxAllowedConcurrentOperations).field("timeslotSize", &self.timeslotSize).field("maxAllowedOperationsPerTimeslot", &self.maxAllowedOperationsPerTimeslot).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_AUTHZ_QUOTA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_AUTHZ_QUOTA {
    fn eq(&self, other: &Self) -> bool {
        self.maxAllowedConcurrentShells == other.maxAllowedConcurrentShells && self.maxAllowedConcurrentOperations == other.maxAllowedConcurrentOperations && self.timeslotSize == other.timeslotSize && self.maxAllowedOperationsPerTimeslot == other.maxAllowedOperationsPerTimeslot
    }
}
impl ::core::cmp::Eq for WSMAN_AUTHZ_QUOTA {}
impl ::core::default::Default for WSMAN_AUTHZ_QUOTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_CERTIFICATE_DETAILS {
    pub subject: ::windows_core::PCWSTR,
    pub issuerName: ::windows_core::PCWSTR,
    pub issuerThumbprint: ::windows_core::PCWSTR,
    pub subjectName: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_CERTIFICATE_DETAILS {}
impl ::core::clone::Clone for WSMAN_CERTIFICATE_DETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_CERTIFICATE_DETAILS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_CERTIFICATE_DETAILS").field("subject", &self.subject).field("issuerName", &self.issuerName).field("issuerThumbprint", &self.issuerThumbprint).field("subjectName", &self.subjectName).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_CERTIFICATE_DETAILS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_CERTIFICATE_DETAILS {
    fn eq(&self, other: &Self) -> bool {
        self.subject == other.subject && self.issuerName == other.issuerName && self.issuerThumbprint == other.issuerThumbprint && self.subjectName == other.subjectName
    }
}
impl ::core::cmp::Eq for WSMAN_CERTIFICATE_DETAILS {}
impl ::core::default::Default for WSMAN_CERTIFICATE_DETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_COMMAND_ARG_SET {
    pub argsCount: u32,
    pub args: *const ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_COMMAND_ARG_SET {}
impl ::core::clone::Clone for WSMAN_COMMAND_ARG_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_COMMAND_ARG_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_COMMAND_ARG_SET").field("argsCount", &self.argsCount).field("args", &self.args).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_COMMAND_ARG_SET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_COMMAND_ARG_SET {
    fn eq(&self, other: &Self) -> bool {
        self.argsCount == other.argsCount && self.args == other.args
    }
}
impl ::core::cmp::Eq for WSMAN_COMMAND_ARG_SET {}
impl ::core::default::Default for WSMAN_COMMAND_ARG_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSMAN_COMMAND_HANDLE(pub isize);
impl ::core::default::Default for WSMAN_COMMAND_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for WSMAN_COMMAND_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for WSMAN_COMMAND_HANDLE {}
impl ::core::fmt::Debug for WSMAN_COMMAND_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSMAN_COMMAND_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_COMMAND_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct WSMAN_CONNECT_DATA {
    pub data: WSMAN_DATA,
}
impl ::core::marker::Copy for WSMAN_CONNECT_DATA {}
impl ::core::clone::Clone for WSMAN_CONNECT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_CONNECT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_CONNECT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_CREATE_SHELL_DATA {
    pub data: WSMAN_DATA,
}
impl ::core::marker::Copy for WSMAN_CREATE_SHELL_DATA {}
impl ::core::clone::Clone for WSMAN_CREATE_SHELL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_CREATE_SHELL_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_CREATE_SHELL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_DATA {
    pub r#type: WSManDataType,
    pub Anonymous: WSMAN_DATA_0,
}
impl ::core::marker::Copy for WSMAN_DATA {}
impl ::core::clone::Clone for WSMAN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WSMAN_DATA_0 {
    pub text: WSMAN_DATA_TEXT,
    pub binaryData: WSMAN_DATA_BINARY,
    pub number: u32,
}
impl ::core::marker::Copy for WSMAN_DATA_0 {}
impl ::core::clone::Clone for WSMAN_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_DATA_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_DATA_BINARY {
    pub dataLength: u32,
    pub data: *mut u8,
}
impl ::core::marker::Copy for WSMAN_DATA_BINARY {}
impl ::core::clone::Clone for WSMAN_DATA_BINARY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_DATA_BINARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_DATA_BINARY").field("dataLength", &self.dataLength).field("data", &self.data).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_DATA_BINARY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_DATA_BINARY {
    fn eq(&self, other: &Self) -> bool {
        self.dataLength == other.dataLength && self.data == other.data
    }
}
impl ::core::cmp::Eq for WSMAN_DATA_BINARY {}
impl ::core::default::Default for WSMAN_DATA_BINARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_DATA_TEXT {
    pub bufferLength: u32,
    pub buffer: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_DATA_TEXT {}
impl ::core::clone::Clone for WSMAN_DATA_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_DATA_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_DATA_TEXT").field("bufferLength", &self.bufferLength).field("buffer", &self.buffer).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_DATA_TEXT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_DATA_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.bufferLength == other.bufferLength && self.buffer == other.buffer
    }
}
impl ::core::cmp::Eq for WSMAN_DATA_TEXT {}
impl ::core::default::Default for WSMAN_DATA_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_ENVIRONMENT_VARIABLE {
    pub name: ::windows_core::PCWSTR,
    pub value: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_ENVIRONMENT_VARIABLE {}
impl ::core::clone::Clone for WSMAN_ENVIRONMENT_VARIABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_ENVIRONMENT_VARIABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_ENVIRONMENT_VARIABLE").field("name", &self.name).field("value", &self.value).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_ENVIRONMENT_VARIABLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_ENVIRONMENT_VARIABLE {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::core::cmp::Eq for WSMAN_ENVIRONMENT_VARIABLE {}
impl ::core::default::Default for WSMAN_ENVIRONMENT_VARIABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_ENVIRONMENT_VARIABLE_SET {
    pub varsCount: u32,
    pub vars: *mut WSMAN_ENVIRONMENT_VARIABLE,
}
impl ::core::marker::Copy for WSMAN_ENVIRONMENT_VARIABLE_SET {}
impl ::core::clone::Clone for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_ENVIRONMENT_VARIABLE_SET").field("varsCount", &self.varsCount).field("vars", &self.vars).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_ENVIRONMENT_VARIABLE_SET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.varsCount == other.varsCount && self.vars == other.vars
    }
}
impl ::core::cmp::Eq for WSMAN_ENVIRONMENT_VARIABLE_SET {}
impl ::core::default::Default for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_ERROR {
    pub code: u32,
    pub errorDetail: ::windows_core::PCWSTR,
    pub language: ::windows_core::PCWSTR,
    pub machineName: ::windows_core::PCWSTR,
    pub pluginName: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_ERROR {}
impl ::core::clone::Clone for WSMAN_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_ERROR").field("code", &self.code).field("errorDetail", &self.errorDetail).field("language", &self.language).field("machineName", &self.machineName).field("pluginName", &self.pluginName).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_ERROR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.errorDetail == other.errorDetail && self.language == other.language && self.machineName == other.machineName && self.pluginName == other.pluginName
    }
}
impl ::core::cmp::Eq for WSMAN_ERROR {}
impl ::core::default::Default for WSMAN_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_FILTER {
    pub filter: ::windows_core::PCWSTR,
    pub dialect: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_FILTER {}
impl ::core::clone::Clone for WSMAN_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_FILTER").field("filter", &self.filter).field("dialect", &self.dialect).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.filter == other.filter && self.dialect == other.dialect
    }
}
impl ::core::cmp::Eq for WSMAN_FILTER {}
impl ::core::default::Default for WSMAN_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_FRAGMENT {
    pub path: ::windows_core::PCWSTR,
    pub dialect: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_FRAGMENT {}
impl ::core::clone::Clone for WSMAN_FRAGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_FRAGMENT").field("path", &self.path).field("dialect", &self.dialect).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_FRAGMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.dialect == other.dialect
    }
}
impl ::core::cmp::Eq for WSMAN_FRAGMENT {}
impl ::core::default::Default for WSMAN_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_KEY {
    pub key: ::windows_core::PCWSTR,
    pub value: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_KEY {}
impl ::core::clone::Clone for WSMAN_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_KEY").field("key", &self.key).field("value", &self.value).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_KEY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}
impl ::core::cmp::Eq for WSMAN_KEY {}
impl ::core::default::Default for WSMAN_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSMAN_OPERATION_HANDLE(pub isize);
impl ::core::default::Default for WSMAN_OPERATION_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for WSMAN_OPERATION_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for WSMAN_OPERATION_HANDLE {}
impl ::core::fmt::Debug for WSMAN_OPERATION_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSMAN_OPERATION_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_OPERATION_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct WSMAN_OPERATION_INFO {
    pub fragment: WSMAN_FRAGMENT,
    pub filter: WSMAN_FILTER,
    pub selectorSet: WSMAN_SELECTOR_SET,
    pub optionSet: WSMAN_OPTION_SET,
    pub reserved: *mut ::core::ffi::c_void,
    pub version: u32,
}
impl ::core::marker::Copy for WSMAN_OPERATION_INFO {}
impl ::core::clone::Clone for WSMAN_OPERATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_OPERATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPERATION_INFO").field("fragment", &self.fragment).field("filter", &self.filter).field("selectorSet", &self.selectorSet).field("optionSet", &self.optionSet).field("reserved", &self.reserved).field("version", &self.version).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_OPERATION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_OPERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fragment == other.fragment && self.filter == other.filter && self.selectorSet == other.selectorSet && self.optionSet == other.optionSet && self.reserved == other.reserved && self.version == other.version
    }
}
impl ::core::cmp::Eq for WSMAN_OPERATION_INFO {}
impl ::core::default::Default for WSMAN_OPERATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_OPERATION_INFOEX {
    pub fragment: WSMAN_FRAGMENT,
    pub filter: WSMAN_FILTER,
    pub selectorSet: WSMAN_SELECTOR_SET,
    pub optionSet: WSMAN_OPTION_SETEX,
    pub version: u32,
    pub uiLocale: ::windows_core::PCWSTR,
    pub dataLocale: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_OPERATION_INFOEX {}
impl ::core::clone::Clone for WSMAN_OPERATION_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_OPERATION_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPERATION_INFOEX").field("fragment", &self.fragment).field("filter", &self.filter).field("selectorSet", &self.selectorSet).field("optionSet", &self.optionSet).field("version", &self.version).field("uiLocale", &self.uiLocale).field("dataLocale", &self.dataLocale).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_OPERATION_INFOEX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_OPERATION_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.fragment == other.fragment && self.filter == other.filter && self.selectorSet == other.selectorSet && self.optionSet == other.optionSet && self.version == other.version && self.uiLocale == other.uiLocale && self.dataLocale == other.dataLocale
    }
}
impl ::core::cmp::Eq for WSMAN_OPERATION_INFOEX {}
impl ::core::default::Default for WSMAN_OPERATION_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_OPTION {
    pub name: ::windows_core::PCWSTR,
    pub value: ::windows_core::PCWSTR,
    pub mustComply: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for WSMAN_OPTION {}
impl ::core::clone::Clone for WSMAN_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPTION").field("name", &self.name).field("value", &self.value).field("mustComply", &self.mustComply).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_OPTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_OPTION {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value && self.mustComply == other.mustComply
    }
}
impl ::core::cmp::Eq for WSMAN_OPTION {}
impl ::core::default::Default for WSMAN_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_OPTION_SET {
    pub optionsCount: u32,
    pub options: *mut WSMAN_OPTION,
    pub optionsMustUnderstand: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for WSMAN_OPTION_SET {}
impl ::core::clone::Clone for WSMAN_OPTION_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_OPTION_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPTION_SET").field("optionsCount", &self.optionsCount).field("options", &self.options).field("optionsMustUnderstand", &self.optionsMustUnderstand).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_OPTION_SET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_OPTION_SET {
    fn eq(&self, other: &Self) -> bool {
        self.optionsCount == other.optionsCount && self.options == other.options && self.optionsMustUnderstand == other.optionsMustUnderstand
    }
}
impl ::core::cmp::Eq for WSMAN_OPTION_SET {}
impl ::core::default::Default for WSMAN_OPTION_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_OPTION_SETEX {
    pub optionsCount: u32,
    pub options: *mut WSMAN_OPTION,
    pub optionsMustUnderstand: super::super::Foundation::BOOL,
    pub optionTypes: *const ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_OPTION_SETEX {}
impl ::core::clone::Clone for WSMAN_OPTION_SETEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_OPTION_SETEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPTION_SETEX").field("optionsCount", &self.optionsCount).field("options", &self.options).field("optionsMustUnderstand", &self.optionsMustUnderstand).field("optionTypes", &self.optionTypes).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_OPTION_SETEX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_OPTION_SETEX {
    fn eq(&self, other: &Self) -> bool {
        self.optionsCount == other.optionsCount && self.options == other.options && self.optionsMustUnderstand == other.optionsMustUnderstand && self.optionTypes == other.optionTypes
    }
}
impl ::core::cmp::Eq for WSMAN_OPTION_SETEX {}
impl ::core::default::Default for WSMAN_OPTION_SETEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_PLUGIN_REQUEST {
    pub senderDetails: *mut WSMAN_SENDER_DETAILS,
    pub locale: ::windows_core::PCWSTR,
    pub resourceUri: ::windows_core::PCWSTR,
    pub operationInfo: *mut WSMAN_OPERATION_INFO,
    pub shutdownNotification: super::super::Foundation::BOOL,
    pub shutdownNotificationHandle: super::super::Foundation::HANDLE,
    pub dataLocale: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_PLUGIN_REQUEST {}
impl ::core::clone::Clone for WSMAN_PLUGIN_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_PLUGIN_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_PLUGIN_REQUEST").field("senderDetails", &self.senderDetails).field("locale", &self.locale).field("resourceUri", &self.resourceUri).field("operationInfo", &self.operationInfo).field("shutdownNotification", &self.shutdownNotification).field("shutdownNotificationHandle", &self.shutdownNotificationHandle).field("dataLocale", &self.dataLocale).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_PLUGIN_REQUEST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_PLUGIN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.senderDetails == other.senderDetails && self.locale == other.locale && self.resourceUri == other.resourceUri && self.operationInfo == other.operationInfo && self.shutdownNotification == other.shutdownNotification && self.shutdownNotificationHandle == other.shutdownNotificationHandle && self.dataLocale == other.dataLocale
    }
}
impl ::core::cmp::Eq for WSMAN_PLUGIN_REQUEST {}
impl ::core::default::Default for WSMAN_PLUGIN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_PROXY_INFO {
    pub accessType: u32,
    pub authenticationCredentials: WSMAN_AUTHENTICATION_CREDENTIALS,
}
impl ::core::marker::Copy for WSMAN_PROXY_INFO {}
impl ::core::clone::Clone for WSMAN_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_PROXY_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_RECEIVE_DATA_RESULT {
    pub streamId: ::windows_core::PCWSTR,
    pub streamData: WSMAN_DATA,
    pub commandState: ::windows_core::PCWSTR,
    pub exitCode: u32,
}
impl ::core::marker::Copy for WSMAN_RECEIVE_DATA_RESULT {}
impl ::core::clone::Clone for WSMAN_RECEIVE_DATA_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_RECEIVE_DATA_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_RECEIVE_DATA_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WSMAN_RESPONSE_DATA {
    pub receiveData: WSMAN_RECEIVE_DATA_RESULT,
    pub connectData: WSMAN_CONNECT_DATA,
    pub createData: WSMAN_CREATE_SHELL_DATA,
}
impl ::core::marker::Copy for WSMAN_RESPONSE_DATA {}
impl ::core::clone::Clone for WSMAN_RESPONSE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WSMAN_RESPONSE_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_RESPONSE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_SELECTOR_SET {
    pub numberKeys: u32,
    pub keys: *mut WSMAN_KEY,
}
impl ::core::marker::Copy for WSMAN_SELECTOR_SET {}
impl ::core::clone::Clone for WSMAN_SELECTOR_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_SELECTOR_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SELECTOR_SET").field("numberKeys", &self.numberKeys).field("keys", &self.keys).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_SELECTOR_SET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_SELECTOR_SET {
    fn eq(&self, other: &Self) -> bool {
        self.numberKeys == other.numberKeys && self.keys == other.keys
    }
}
impl ::core::cmp::Eq for WSMAN_SELECTOR_SET {}
impl ::core::default::Default for WSMAN_SELECTOR_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_SENDER_DETAILS {
    pub senderName: ::windows_core::PCWSTR,
    pub authenticationMechanism: ::windows_core::PCWSTR,
    pub certificateDetails: *mut WSMAN_CERTIFICATE_DETAILS,
    pub clientToken: super::super::Foundation::HANDLE,
    pub httpURL: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_SENDER_DETAILS {}
impl ::core::clone::Clone for WSMAN_SENDER_DETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_SENDER_DETAILS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SENDER_DETAILS").field("senderName", &self.senderName).field("authenticationMechanism", &self.authenticationMechanism).field("certificateDetails", &self.certificateDetails).field("clientToken", &self.clientToken).field("httpURL", &self.httpURL).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_SENDER_DETAILS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_SENDER_DETAILS {
    fn eq(&self, other: &Self) -> bool {
        self.senderName == other.senderName && self.authenticationMechanism == other.authenticationMechanism && self.certificateDetails == other.certificateDetails && self.clientToken == other.clientToken && self.httpURL == other.httpURL
    }
}
impl ::core::cmp::Eq for WSMAN_SENDER_DETAILS {}
impl ::core::default::Default for WSMAN_SENDER_DETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSMAN_SESSION_HANDLE(pub isize);
impl ::core::default::Default for WSMAN_SESSION_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for WSMAN_SESSION_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for WSMAN_SESSION_HANDLE {}
impl ::core::fmt::Debug for WSMAN_SESSION_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSMAN_SESSION_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_SESSION_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct WSMAN_SHELL_ASYNC {
    pub operationContext: *mut ::core::ffi::c_void,
    pub completionFunction: WSMAN_SHELL_COMPLETION_FUNCTION,
}
impl ::core::marker::Copy for WSMAN_SHELL_ASYNC {}
impl ::core::clone::Clone for WSMAN_SHELL_ASYNC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_SHELL_ASYNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SHELL_ASYNC").field("operationContext", &self.operationContext).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_SHELL_ASYNC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WSMAN_SHELL_ASYNC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_SHELL_DISCONNECT_INFO {
    pub idleTimeoutMs: u32,
}
impl ::core::marker::Copy for WSMAN_SHELL_DISCONNECT_INFO {}
impl ::core::clone::Clone for WSMAN_SHELL_DISCONNECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_SHELL_DISCONNECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SHELL_DISCONNECT_INFO").field("idleTimeoutMs", &self.idleTimeoutMs).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_SHELL_DISCONNECT_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_SHELL_DISCONNECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.idleTimeoutMs == other.idleTimeoutMs
    }
}
impl ::core::cmp::Eq for WSMAN_SHELL_DISCONNECT_INFO {}
impl ::core::default::Default for WSMAN_SHELL_DISCONNECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSMAN_SHELL_HANDLE(pub isize);
impl ::core::default::Default for WSMAN_SHELL_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for WSMAN_SHELL_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for WSMAN_SHELL_HANDLE {}
impl ::core::fmt::Debug for WSMAN_SHELL_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSMAN_SHELL_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_SHELL_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct WSMAN_SHELL_STARTUP_INFO_V10 {
    pub inputStreamSet: *mut WSMAN_STREAM_ID_SET,
    pub outputStreamSet: *mut WSMAN_STREAM_ID_SET,
    pub idleTimeoutMs: u32,
    pub workingDirectory: ::windows_core::PCWSTR,
    pub variableSet: *mut WSMAN_ENVIRONMENT_VARIABLE_SET,
}
impl ::core::marker::Copy for WSMAN_SHELL_STARTUP_INFO_V10 {}
impl ::core::clone::Clone for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SHELL_STARTUP_INFO_V10").field("inputStreamSet", &self.inputStreamSet).field("outputStreamSet", &self.outputStreamSet).field("idleTimeoutMs", &self.idleTimeoutMs).field("workingDirectory", &self.workingDirectory).field("variableSet", &self.variableSet).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_SHELL_STARTUP_INFO_V10 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn eq(&self, other: &Self) -> bool {
        self.inputStreamSet == other.inputStreamSet && self.outputStreamSet == other.outputStreamSet && self.idleTimeoutMs == other.idleTimeoutMs && self.workingDirectory == other.workingDirectory && self.variableSet == other.variableSet
    }
}
impl ::core::cmp::Eq for WSMAN_SHELL_STARTUP_INFO_V10 {}
impl ::core::default::Default for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_SHELL_STARTUP_INFO_V11 {
    pub Base: WSMAN_SHELL_STARTUP_INFO_V10,
    pub name: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_SHELL_STARTUP_INFO_V11 {}
impl ::core::clone::Clone for WSMAN_SHELL_STARTUP_INFO_V11 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_SHELL_STARTUP_INFO_V11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SHELL_STARTUP_INFO_V11").field("Base", &self.Base).field("name", &self.name).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_SHELL_STARTUP_INFO_V11 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_SHELL_STARTUP_INFO_V11 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.name == other.name
    }
}
impl ::core::cmp::Eq for WSMAN_SHELL_STARTUP_INFO_V11 {}
impl ::core::default::Default for WSMAN_SHELL_STARTUP_INFO_V11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_STREAM_ID_SET {
    pub streamIDsCount: u32,
    pub streamIDs: *const ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_STREAM_ID_SET {}
impl ::core::clone::Clone for WSMAN_STREAM_ID_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_STREAM_ID_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_STREAM_ID_SET").field("streamIDsCount", &self.streamIDsCount).field("streamIDs", &self.streamIDs).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_STREAM_ID_SET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_STREAM_ID_SET {
    fn eq(&self, other: &Self) -> bool {
        self.streamIDsCount == other.streamIDsCount && self.streamIDs == other.streamIDs
    }
}
impl ::core::cmp::Eq for WSMAN_STREAM_ID_SET {}
impl ::core::default::Default for WSMAN_STREAM_ID_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMAN_USERNAME_PASSWORD_CREDS {
    pub username: ::windows_core::PCWSTR,
    pub password: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_USERNAME_PASSWORD_CREDS {}
impl ::core::clone::Clone for WSMAN_USERNAME_PASSWORD_CREDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSMAN_USERNAME_PASSWORD_CREDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_USERNAME_PASSWORD_CREDS").field("username", &self.username).field("password", &self.password).finish()
    }
}
impl ::windows_core::TypeKind for WSMAN_USERNAME_PASSWORD_CREDS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WSMAN_USERNAME_PASSWORD_CREDS {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username && self.password == other.password
    }
}
impl ::core::cmp::Eq for WSMAN_USERNAME_PASSWORD_CREDS {}
impl ::core::default::Default for WSMAN_USERNAME_PASSWORD_CREDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WSMAN_PLUGIN_AUTHORIZE_OPERATION = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, operation: u32, action: ::windows_core::PCWSTR, resourceuri: ::windows_core::PCWSTR)>;
pub type WSMAN_PLUGIN_AUTHORIZE_QUERY_QUOTA = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32)>;
pub type WSMAN_PLUGIN_AUTHORIZE_RELEASE_CONTEXT = ::core::option::Option<unsafe extern "system" fn(userauthorizationcontext: *const ::core::ffi::c_void)>;
pub type WSMAN_PLUGIN_AUTHORIZE_USER = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32)>;
pub type WSMAN_PLUGIN_COMMAND = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandline: ::windows_core::PCWSTR, arguments: *const WSMAN_COMMAND_ARG_SET)>;
pub type WSMAN_PLUGIN_CONNECT = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void, inboundconnectinformation: *const WSMAN_DATA)>;
pub type WSMAN_PLUGIN_RECEIVE = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void, streamset: *const WSMAN_STREAM_ID_SET)>;
pub type WSMAN_PLUGIN_RELEASE_COMMAND_CONTEXT = ::core::option::Option<unsafe extern "system" fn(shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void)>;
pub type WSMAN_PLUGIN_RELEASE_SHELL_CONTEXT = ::core::option::Option<unsafe extern "system" fn(shellcontext: *const ::core::ffi::c_void)>;
pub type WSMAN_PLUGIN_SEND = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void, stream: ::windows_core::PCWSTR, inbounddata: *const WSMAN_DATA)>;
pub type WSMAN_PLUGIN_SHELL = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, startupinfo: *const WSMAN_SHELL_STARTUP_INFO_V11, inboundshellinformation: *const WSMAN_DATA)>;
pub type WSMAN_PLUGIN_SHUTDOWN = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, flags: u32, reason: u32) -> u32>;
pub type WSMAN_PLUGIN_SIGNAL = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void, code: ::windows_core::PCWSTR)>;
pub type WSMAN_PLUGIN_STARTUP = ::core::option::Option<unsafe extern "system" fn(flags: u32, applicationidentification: ::windows_core::PCWSTR, extrainfo: ::windows_core::PCWSTR, plugincontext: *mut *mut ::core::ffi::c_void) -> u32>;
pub type WSMAN_SHELL_COMPLETION_FUNCTION = ::core::option::Option<unsafe extern "system" fn(operationcontext: *const ::core::ffi::c_void, flags: u32, error: *const WSMAN_ERROR, shell: WSMAN_SHELL_HANDLE, command: WSMAN_COMMAND_HANDLE, operationhandle: WSMAN_OPERATION_HANDLE, data: *const WSMAN_RESPONSE_DATA)>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
