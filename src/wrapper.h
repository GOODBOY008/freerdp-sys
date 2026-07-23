/**
 * wrapper.h — bindgen input header for freerdp-sys
 *
 * This file includes the core FreeRDP 3.x public headers that we want
 * to generate Rust FFI bindings for. The build system passes the correct
 * include paths via -I flags.
 *
 * FreeRDP 3.x header layout:
 *   <prefix>/include/freerdp3/freerdp/...
 *   <prefix>/include/freerdp3/winpr/...
 */

#ifndef FREERDP_SYS_WRAPPER_H
#define FREERDP_SYS_WRAPPER_H

/* WinPR — platform abstraction layer (types, streams, crypto, etc.) */
#include <winpr/winpr.h>
#include <winpr/wtypes.h>
#include <winpr/stream.h>
#include <winpr/crt.h>
#include <winpr/string.h>
#include <winpr/memory.h>
#include <winpr/crypto.h>
#include <winpr/ssl.h>
#include <winpr/library.h>
#include <winpr/path.h>
#include <winpr/error.h>
#include <winpr/handle.h>
#include <winpr/synch.h>
#include <winpr/thread.h>
#include <winpr/environment.h>
#include <winpr/cmdline.h>
#include <winpr/settings.h>
#include <winpr/wlog.h>

/* FreeRDP core */
#include <freerdp/freerdp.h>
#include <freerdp/api.h>
#include <freerdp/types.h>
#include <freerdp/settings.h>
#include <freerdp/constants.h>
#include <freerdp/error.h>
#include <freerdp/event.h>
#include <freerdp/client.h>
#include <freerdp/channels/channels.h>

/* Connection and authentication */
#include <freerdp/client/channels.h>
#include <freerdp/client/cmdline.h>

/* Graphics / display */
#include <freerdp/gdi/gdi.h>
#include <freerdp/codec/color.h>
#include <freerdp/codec/region.h>

/* Input */
#include <freerdp/input.h>
#include <freerdp/scancode.h>

/* Primary/secondary drawing orders */
#include <freerdp/primary.h>

/* Certificate / TLS verification callbacks */
#include <freerdp/crypto/crypto.h>

#endif /* FREERDP_SYS_WRAPPER_H */
