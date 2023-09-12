9.2. Adding a basic dissector
Prev    Chapter 9. Packet Dissection     Next
9.2. Adding a basic dissector

Let’s step through adding a basic dissector. We’ll start with the made up "foo" protocol. It consists of the following basic items.

    A packet type - 8 bits. Possible values: 1 - initialisation, 2 - terminate, 3 - data.
    A set of flags stored in 8 bits. 0x01 - start packet, 0x02 - end packet, 0x04 - priority packet.
    A sequence number - 16 bits.
    An IPv4 address.

## 9.2.1. Setting up the dissector

The first decision you need to make is if this dissector will be a built-in dissector and included in the main program, or a plugin.

Plugins are easier to write initially, so let’s start with that. With a little care, the plugin can be converted into a built-in dissector.

### Dissector Initialisation

```c
#include "config.h"
#include <epan/packet.h>

#define FOO_PORT 1234

static int proto_foo = -1;

void
proto_register_foo(void)
{
    proto_foo = proto_register_protocol (
        "FOO Protocol", /* name        */
        "FOO",          /* short name  */
        "foo"           /* filter_name */
        );
}
```

Let’s go through this a bit at a time. First we have some boilerplate include files. These will be pretty constant to start with.

Then a #define for the UDP port that carries foo traffic.

Next we have proto_foo, an int that stores our protocol handle and is initialised to -1. This handle will be set when the dissector is registered within the main program. It’s good practice to make all variables and functions that aren’t exported static to minimize name space pollution. This normally isn’t a problem unless your dissector gets so big that it spans multiple files.

Now that we have the basics in place to interact with the main program, we’ll start with two protocol dissector setup functions: proto_register_XXX and proto_reg_handoff_XXX.

Each protocol must have a register function with the form "proto_register_XXX". This function is used to register the protocol in Wireshark. The code to call the register routines is generated automatically and is called when Wireshark starts. In this example, the function is named proto_register_foo.

proto_register_foo calls proto_register_protocol(), which takes a name, short name, and filter_name. The name and short name are used in the "Preferences" and "Enabled protocols" dialogs and the documentation’s generated field name list. The filter_name is used as the display filter name. proto_register_protocol() returns a protocol handle, which can be used to refer to the protocol and obtain a handle to the protocol’s dissector.

Next we need a handoff routine.

### Dissector Handoff

```c
void
proto_reg_handoff_foo(void)
{
    static dissector_handle_t foo_handle;

    foo_handle = create_dissector_handle(dissect_foo, proto_foo);
    dissector_add_uint("udp.port", FOO_PORT, foo_handle);
}
```

A handoff routine associates a protocol handler with the protocol’s traffic. It consists of two major steps: The first step is to create a dissector handle, which is a handle associated with the protocol and the function called to do the actual dissecting. The second step is to register the dissector handle so that traffic associated with the protocol calls the dissector.

In this example, proto_reg_handoff_foo() calls create_dissector_handle() to obtain a dissector handle for the foo protocol. It then uses dissector_add_uint() to associate traffic on UDP port FOO_PORT (1234) with the foo protocol, so that Wireshark will call dissect_foo() when it receives UDP traffic on port 1234.

Wireshark’s dissector convention is to put proto_register_foo() and proto_reg_handoff_foo() as the last two functions in the dissector source.

The next step is to write the dissecting function, dissect_foo(). We’ll start with a basic placeholder.

Dissection.

```c
static int
dissect_foo(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree _U_, void *data _U_)
{
    col_set_str(pinfo->cinfo, COL_PROTOCOL, "FOO");
    /* Clear the info column */
    col_clear(pinfo->cinfo,COL_INFO);

    return tvb_captured_length(tvb);
}
```

dissect_foo() is called to dissect the packets presented to it. The packet data is held in a special buffer referenced here as tvb. The packet_info structure contains general data about the protocol and we can update information here. The tree parameter is where the detail dissection takes place. Note that the _U_ following tree and data signals to the compiler that the parameters are unused, so that the compiler does not print a warning.

For now we’ll do the minimum we can get away with. col_set_str() is used to set Wireshark’s Protocol column to "FOO" so everyone can see it’s being recognised. The only other thing we do is to clear out any data in the INFO column if it’s being displayed.

At this point we have a basic dissector ready to compile and install. The dissector doesn’t do anything other than identify the protocol and label it. Here is the dissector’s complete code:

Complete packet-foo.c:.

```c
#include "config.h"
#include <epan/packet.h>

#define FOO_PORT 1234

static int proto_foo = -1;

static int
dissect_foo(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree _U_, void *data _U_)
{
    col_set_str(pinfo->cinfo, COL_PROTOCOL, "FOO");
    /* Clear the info column */
    col_clear(pinfo->cinfo,COL_INFO);

    return tvb_captured_length(tvb);
}

void
proto_register_foo(void)
{
    proto_foo = proto_register_protocol (
        "FOO Protocol", /* name        */
        "FOO",          /* short_name  */
        "foo"           /* filter_name */
        );
}

void
proto_reg_handoff_foo(void)
{
    static dissector_handle_t foo_handle;

    foo_handle = create_dissector_handle(dissect_foo, proto_foo);
    dissector_add_uint("udp.port", FOO_PORT, foo_handle);
}
```

To compile this dissector and create a plugin a few support files are required, besides the dissector source in packet-foo.c:

    CMakeLists.txt - Contains the CMake file and version info for this plugin.
    packet-foo.c - Your dissector source.
    plugin.rc.in - Contains the DLL resource template for Windows. (optional)

Samples of these files are available in the gryphon plugin directory (plugins/epan/gryphon). If you copy the files from the gryphon plugin, CMakeLists.txt will need to be updated with the correct plugin name, version info, and the relevant files to compile.

In the main top-level source directory, copy CMakeListsCustom.txt.example to CMakeListsCustom.txt and add the path of your plugin to the list in CUSTOM_PLUGIN_SRC_DIR.

Compile the dissector to a DLL or shared library and either run Wireshark from the build directory as detailed in Section 3.7, “Run Your Version Of Wireshark” or copy the plugin binary into the plugin directory of your Wireshark installation and run that.
9.2.2. Dissecting the protocol’s details

## Dissection

Now that we have our basic dissector up and running, let’s do something with it. The simplest thing to start with is labeling the payload. We can label the payload by building a subtree to decode our results into. This subtree will hold all the protocol’s details and helps keep things looking nice in the detailed display.

We add the new subtree with `proto_tree_add_item()`, as is depicted below:

Plugin Packet Dissection.

```c
static int
dissect_foo(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree, void *data _U_)
{
    col_set_str(pinfo->cinfo, COL_PROTOCOL, "FOO");
    /* Clear out stuff in the info column */
    col_clear(pinfo->cinfo,COL_INFO);

    proto_item *ti = proto_tree_add_item(tree, proto_foo, tvb, 0, -1, ENC_NA);

    return tvb_captured_length(tvb);
}
```

As the FOO protocol does not encapsulate another protocol, we consume all of the tvb’s data, from 0 to the end (-1).

The final parameter specifies the "encoding" and is set to ENC_NA ("not applicable"), as the protocol doesn’t specifically use big endian (ENC_BIG_ENDIAN) or little endian (ENC_LITTLE_ENDIAN).

After adding the call to proto_tree_add_item() , there should be a label FOO in the protocol’s detailed display. Selecting this label will highlight the remaining contents of the packet.

Now let’s go to the next step and add some protocol dissection. To do this we’ll need to construct tables to define which fields will be present in the packet and to store the opened/closed state of the subtree. We’ll add these statically allocated arrays to the beginning of the file and name them hf_register_info ('hf' is short for 'header field') and ett. The arrays wil then registered after the call to proto_register_protocol() by calling proto_register_field_array() and proto_register_subtree_array():

### Registering data structures

```c
static int hf_foo_pdu_type = -1;
static int ett_foo = -1;

/* ... */

void
proto_register_foo(void)
{
    static hf_register_info hf[] = {
        { &hf_foo_pdu_type,
            { "FOO PDU Type", "foo.type",
            FT_UINT8, BASE_DEC,
            NULL, 0x0,
            NULL, HFILL }
        }
    };

    /* Setup protocol subtree array */
    static int *ett[] = {
        &ett_foo
    };

    proto_foo = proto_register_protocol (
        "FOO Protocol", /* name       */
        "FOO",          /* short_name */
        "foo"           /* filter_name*/
        );

    proto_register_field_array(proto_foo, hf, array_length(hf));
    proto_register_subtree_array(ett, array_length(ett));
}
```

As you can see, a field foo.type was defined inside the array of header fields.

Now we can dissect the FOO PDU Type (referenced as foo.type) field in dissect_foo() by adding the FOO Protocol’s subtree with proto_item_add_subtree() and then calling proto_tree_add_item() to add the field:

Dissector starting to dissect the packets.

```c
    proto_item *ti = proto_tree_add_item(tree, proto_foo, tvb, 0, -1, ENC_NA);
    proto_tree *foo_tree = proto_item_add_subtree(ti, ett_foo);
    proto_tree_add_item(foo_tree, hf_foo_pdu_type, tvb, 0, 1, ENC_BIG_ENDIAN);
```

As mentioned earlier, the foo protocol begins with an 8-bit packet type which can have three possible values: 1 - initialisation, 2 - terminate, 3 - data. Here’s how we can add the packet details:

The proto_item_add_subtree() call has added a child node to the protocol tree which is where we will do our detail dissection. The expansion of this node is controlled by the ett_foo variable. This remembers if the node should be expanded or not as you move between packets. All subsequent dissection will be added to this tree, as you can see from the next call. A call to proto_tree_add_item() in the foo_tree, this time using the hf_foo_pdu_type to control the formatting of the item. The pdu type is one byte of data, starting at 0. We assume it is in network order (also called big endian), so that is why we use ENC_BIG_ENDIAN. For a 1-byte quantity, there is no order issue, but it is good practice to make this the same as any multibyte fields that may be present, and as we will see in the next section, this particular protocol uses network order.

If we look in detail at the hf_foo_pdu_type declaration in the static array we can see the details of the definition.

```c
static hf_register_info hf[] = {
    { &hf_foo_pdu_type,
        { "FOO PDU Type", "foo.type",
        FT_UINT8, BASE_DEC,
        NULL, 0x0,
        NULL, HFILL }
    }
};
```

    hf_foo_pdu_type - The node’s index.
    FOO PDU Type - The item’s label.
    foo.type - The item’s abbreviated name, for use in the display filter (e.g., foo.type=1).
    FT_UINT8 - The item’s type: An 8bit unsigned integer. This tallies with our call above where we tell it to only look at one byte.
    BASE_DEC - For an integer type, this tells it to be printed as a decimal number. It could be hexadecimal (BASE_HEX) or octal (BASE_OCT) if that made more sense.

We’ll ignore the rest of the structure for now.

If you install this plugin and try it out, you’ll see something that begins to look useful.

Now let’s finish off dissecting the simple protocol. We need to add a few more variables to the hfarray, and a couple more procedure calls.

Wrapping up the packet dissection.

```c
...
static int hf_foo_flags = -1;
static int hf_foo_sequenceno = -1;
static int hf_foo_initialip = -1;
...

static int
dissect_foo(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree, void *data _U_)
{
    int offset = 0;

    ...
    proto_item *ti = proto_tree_add_item(tree, proto_foo, tvb, 0, -1, ENC_NA);
    proto_tree *foo_tree = proto_item_add_subtree(ti, ett_foo);
    proto_tree_add_item(foo_tree, hf_foo_pdu_type, tvb, offset, 1, ENC_BIG_ENDIAN);
    offset += 1;
    proto_tree_add_item(foo_tree, hf_foo_flags, tvb, offset, 1, ENC_BIG_ENDIAN);
    offset += 1;
    proto_tree_add_item(foo_tree, hf_foo_sequenceno, tvb, offset, 2, ENC_BIG_ENDIAN);
    offset += 2;
    proto_tree_add_item(foo_tree, hf_foo_initialip, tvb, offset, 4, ENC_BIG_ENDIAN);
    offset += 4;
    ...

    return tvb_captured_length(tvb);
}

void
proto_register_foo(void) {
    ...
        ...
        { &hf_foo_flags,
            { "FOO PDU Flags", "foo.flags",
            FT_UINT8, BASE_HEX,
            NULL, 0x0,
            NULL, HFILL }
        },
        { &hf_foo_sequenceno,
            { "FOO PDU Sequence Number", "foo.seqn",
            FT_UINT16, BASE_DEC,
            NULL, 0x0,
            NULL, HFILL }
        },
        { &hf_foo_initialip,
            { "FOO PDU Initial IP", "foo.initialip",
            FT_IPv4, BASE_NONE,
            NULL, 0x0,
            NULL, HFILL }
        },
        ...
    ...
}
...
```

This dissects all the bits of this simple hypothetical protocol. We’ve introduced a new variable offsetinto the mix to help keep track of where we are in the packet dissection. With these extra bits in place, the whole protocol is now dissected.
9.2.3. Improving the dissection information

We can certainly improve the display of the protocol with a bit of extra data. The first step is to add some text labels. Let’s start by labeling the packet types. There is some useful support for this sort of thing by adding a couple of extra things. First we add a simple table of type to name.

Naming the packet types.

```c
static const value_string packettypenames[] = {
    { 1, "Initialise" },
    { 2, "Terminate" },
    { 3, "Data" },
    { 0, NULL }
};
```

This is a handy data structure that can be used to look up a name for a value. There are routines to directly access this lookup table, but we don’t need to do that, as the support code already has that added in. We just have to give these details to the appropriate part of the data, using the VALS macro.

Adding Names to the protocol.

```c
   { &hf_foo_pdu_type,
        { "FOO PDU Type", "foo.type",
        FT_UINT8, BASE_DEC,
        VALS(packettypenames), 0x0,
        NULL, HFILL }
    }
```

This helps in deciphering the packets, and we can do a similar thing for the flags structure. For this we need to add some more data to the table though.

Adding Flags to the protocol.

```c
#define FOO_START_FLAG      0x01
#define FOO_END_FLAG        0x02
#define FOO_PRIORITY_FLAG   0x04

static int hf_foo_startflag = -1;
static int hf_foo_endflag = -1;
static int hf_foo_priorityflag = -1;

static int
dissect_foo(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree, void *data _U_)
{
    ...
        ...
        static int* const bits[] = {
            &hf_foo_startflag,
            &hf_foo_endflag,
            &hf_foo_priorityflag,
            NULL
        };

        proto_tree_add_bitmask(foo_tree, tvb, offset, hf_foo_flags, ett_foo, bits, ENC_BIG_ENDIAN);
        offset += 1;
        ...
    ...
    return tvb_captured_length(tvb);
}

void
proto_register_foo(void) {
    ...
        ...
        { &hf_foo_startflag,
            { "FOO PDU Start Flags", "foo.flags.start",
            FT_BOOLEAN, 8,
            NULL, FOO_START_FLAG,
            NULL, HFILL }
        },
        { &hf_foo_endflag,
            { "FOO PDU End Flags", "foo.flags.end",
            FT_BOOLEAN, 8,
            NULL, FOO_END_FLAG,
            NULL, HFILL }
        },
        { &hf_foo_priorityflag,
            { "FOO PDU Priority Flags", "foo.flags.priority",
            FT_BOOLEAN, 8,
            NULL, FOO_PRIORITY_FLAG,
            NULL, HFILL }
        },
        ...
    ...
}
...
```

Some things to note here. For the flags, as each bit is a different flag, we use the type FT_BOOLEAN, as the flag is either on or off. Second, we include the flag mask in the 7th field of the data, which allows the system to mask the relevant bit. We’ve also changed the 5th field to 8, to indicate that we are looking at an 8 bit quantity when the flags are extracted. Then finally we add the extra constructs to the dissection routine.

This is starting to look fairly full featured now, but there are a couple of other things we can do to make things look even more pretty. At the moment our dissection shows the packets as "Foo Protocol" which whilst correct is a little uninformative. We can enhance this by adding a little more detail. First, let’s get hold of the actual value of the protocol type. We can use the handy function tvb_get_guint8() to do this. With this value in hand, there are a couple of things we can do. First we can set the INFO column of the non-detailed view to show what sort of PDU it is - which is extremely helpful when looking at protocol traces. Second, we can also display this information in the dissection window.

Enhancing the display.

```c
static int
dissect_foo(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree, void *data _U_)
{
    int offset = 0;
    uint8_t packet_type = tvb_get_guint8(tvb, 0);

    col_set_str(pinfo->cinfo, COL_PROTOCOL, "FOO");
    /* Clear out stuff in the info column */
    col_clear(pinfo->cinfo,COL_INFO);
    col_add_fstr(pinfo->cinfo, COL_INFO, "Type %s",
             val_to_str(packet_type, packettypenames, "Unknown (0x%02x)"));

    proto_item *ti = proto_tree_add_item(tree, proto_foo, tvb, 0, -1, ENC_NA);
    proto_item_append_text(ti, ", Type %s",
        val_to_str(packet_type, packettypenames, "Unknown (0x%02x)"));
    proto_tree *foo_tree = proto_item_add_subtree(ti, ett_foo);
    proto_tree_add_item(foo_tree, hf_foo_pdu_type, tvb, offset, 1, ENC_BIG_ENDIAN);
    offset += 1;

    return tvb_captured_length(tvb);
}
```

So here, after grabbing the value of the first 8 bits, we use it with one of the built-in utility routines val_to_str(), to lookup the value. If the value isn’t found we provide a fallback which just prints the value in hex. We use this twice, once in the INFO field of the columns — if it’s displayed, and similarly we append this data to the base of our dissecting tree.
Prev    Up   Next
Chapter 9. Packet Dissection    Home     9.3. How to add an expert item


## Abbreviation

- ett: epan tree type
- hf: header field
- tvb: testy virtual buffer

## Functions

- proto_tree_add_item


## Header Field Registration

```rust
/// Registers a hf index.
pub fn register_hf_index(args: &RegisterArgs, default_display: c_int, default_type: c_uint) -> c_int {
    let hf_index_ptr = Box::leak(Box::new(-1)) as *mut _;
    let abbrev =
        Box::leak(CString::new(args.prefix).unwrap().into_boxed_c_str()).as_ptr() as *const c_char;
    let type_ = args.ws_type.unwrap_or(default_type);
    let display = args.ws_display.unwrap_or(default_display);

    let hf_register_info = epan_sys::hf_register_info {
        p_id: hf_index_ptr,
        hfinfo: epan_sys::header_field_info {
            name: args.name,
            abbrev,
            type_,
            display,
            strings: std::ptr::null(),
            bitmask: 0,
            blurb: args.blurb,
            id: -1,
            parent: 0,
            ref_type: epan_sys::hf_ref_type_HF_REF_TYPE_NONE,
            same_name_prev_id: -1,
            same_name_next: std::ptr::null_mut(),
        },
    };
    let hfs = Box::leak(Box::new([hf_register_info])) as *mut _;

    unsafe {
        epan_sys::proto_register_field_array(args.proto_id, hfs, 1);
    }
    debug_assert_ne!(unsafe { *hf_index_ptr }, -1);
    unsafe { *hf_index_ptr }
}
```

### add_to_tree_single_field/epan_sys::proto_tree_add_item

This one will call let the wireshark do the dissection on the tvb according to given size.
```rust
/// Adds a single field to the protocol tree. Internally, this uses the most basic
/// `proto_tree_add_item` function.
fn add_to_tree_single_field(args: &DissectorArgs, size: usize, default_enc: u32) {
    let hf_index = args.get_hf_index().unwrap();
    unsafe {
        epan_sys::proto_tree_add_item(
            args.parent,
            hf_index,
            args.tvb,
            args.offset as _,
            size as _,
            args.ws_enc.unwrap_or(default_enc),
        );
    }
}
```

### epan_sys::proto_tree_add_*_format_value

These functions are used to display in a way of "value(string)".

```rust
/// Adds a uint type (u8, u16, etc.) to the protocol tree with a custom string.
fn add_to_tree_format_value_uint(
    args: &DissectorArgs,
    size: usize,
    value: c_uint,
    s: &impl std::fmt::Display,
) {
    let hf_index = args.get_hf_index().unwrap();
    let fmt = CString::new(ToString::to_string(s)).unwrap();
    unsafe {
        epan_sys::proto_tree_add_uint_format_value(
            args.parent,
            hf_index,
            args.tvb,
            args.offset as _,
            size as _,
            value,
            fmt.as_ptr(),
        );
    }
}

/// Adds an int type (i8, i16, etc.) to the protocol tree with a custom string.
fn add_to_tree_format_value_int(
    args: &DissectorArgs,
    size: usize,
    value: c_int,
    s: &impl std::fmt::Display,
) {
    let hf_index = args.get_hf_index().unwrap();
    let fmt = CString::new(ToString::to_string(s)).unwrap();
    unsafe {
        epan_sys::proto_tree_add_int_format_value(
            args.parent,
            hf_index,
            args.tvb,
            args.offset as _,
            size as _,
            value,
            fmt.as_ptr(),
        );
    }
}

```

## Plan for zenoh-dissector-rs

- Regularize all the outputs to string (prefer `as_str` or `to_string`), so that we can easily implement all the functions.
- For bytes data, either simply printing them via `Debug` or implementing a proper way to show.
- For `Option` types, skipping them or printing them as `None`
- Properly setup the subtree branch
- Intrusive way: using procedural macros
