// automatically generated by the FlatBuffers compiler, do not modify



use crate::HandInput::*;
use crate::PointerInput::*;
use crate::common::*;
use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod stardust_xr {

  use crate::HandInput::*;
  use crate::PointerInput::*;
  use crate::common::*;
  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_INPUT_DATA_RAW: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_INPUT_DATA_RAW: u8 = 2;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_INPUT_DATA_RAW: [InputDataRaw; 3] = [
  InputDataRaw::NONE,
  InputDataRaw::Pointer,
  InputDataRaw::Hand,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct InputDataRaw(pub u8);
#[allow(non_upper_case_globals)]
impl InputDataRaw {
  pub const NONE: Self = Self(0);
  pub const Pointer: Self = Self(1);
  pub const Hand: Self = Self(2);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::Pointer,
    Self::Hand,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::Pointer => Some("Pointer"),
      Self::Hand => Some("Hand"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for InputDataRaw {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for InputDataRaw {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for InputDataRaw {
    type Output = InputDataRaw;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for InputDataRaw {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for InputDataRaw {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for InputDataRaw {}
pub struct InputDataRawUnionTableOffset {}

pub enum InputDataOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct InputData<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for InputData<'a> {
  type Inner = InputData<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> InputData<'a> {
  pub const VT_UUID: flatbuffers::VOffsetT = 4;
  pub const VT_INPUT_TYPE: flatbuffers::VOffsetT = 6;
  pub const VT_INPUT: flatbuffers::VOffsetT = 8;
  pub const VT_DISTANCE: flatbuffers::VOffsetT = 10;
  pub const VT_DATAMAP: flatbuffers::VOffsetT = 12;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    InputData { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args InputDataArgs<'args>
  ) -> flatbuffers::WIPOffset<InputData<'bldr>> {
    let mut builder = InputDataBuilder::new(_fbb);
    if let Some(x) = args.datamap { builder.add_datamap(x); }
    builder.add_distance(args.distance);
    if let Some(x) = args.input { builder.add_input(x); }
    if let Some(x) = args.uuid { builder.add_uuid(x); }
    builder.add_input_type(args.input_type);
    builder.finish()
  }


  #[inline]
  pub fn uuid(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(InputData::VT_UUID, None).unwrap()
  }
  #[inline]
  pub fn input_type(&self) -> InputDataRaw {
    self._tab.get::<InputDataRaw>(InputData::VT_INPUT_TYPE, Some(InputDataRaw::NONE)).unwrap()
  }
  #[inline]
  pub fn input(&self) -> flatbuffers::Table<'a> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(InputData::VT_INPUT, None).unwrap()
  }
  #[inline]
  pub fn distance(&self) -> f32 {
    self._tab.get::<f32>(InputData::VT_DISTANCE, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn datamap(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(InputData::VT_DATAMAP, None).map(|v| v.safe_slice())
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn input_as_pointer(&self) -> Option<Pointer<'a>> {
    if self.input_type() == InputDataRaw::Pointer {
      let u = self.input();
      Some(Pointer::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn input_as_hand(&self) -> Option<Hand<'a>> {
    if self.input_type() == InputDataRaw::Hand {
      let u = self.input();
      Some(Hand::init_from_table(u))
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for InputData<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("uuid", Self::VT_UUID, true)?
     .visit_union::<InputDataRaw, _>("input_type", Self::VT_INPUT_TYPE, "input", Self::VT_INPUT, true, |key, v, pos| {
        match key {
          InputDataRaw::Pointer => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Pointer>>("InputDataRaw::Pointer", pos),
          InputDataRaw::Hand => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Hand>>("InputDataRaw::Hand", pos),
          _ => Ok(()),
        }
     })?
     .visit_field::<f32>("distance", Self::VT_DISTANCE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("datamap", Self::VT_DATAMAP, false)?
     .finish();
    Ok(())
  }
}
pub struct InputDataArgs<'a> {
    pub uuid: Option<flatbuffers::WIPOffset<&'a str>>,
    pub input_type: InputDataRaw,
    pub input: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
    pub distance: f32,
    pub datamap: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for InputDataArgs<'a> {
  #[inline]
  fn default() -> Self {
    InputDataArgs {
      uuid: None, // required field
      input_type: InputDataRaw::NONE,
      input: None, // required field
      distance: 0.0,
      datamap: None,
    }
  }
}

pub struct InputDataBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> InputDataBuilder<'a, 'b> {
  #[inline]
  pub fn add_uuid(&mut self, uuid: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(InputData::VT_UUID, uuid);
  }
  #[inline]
  pub fn add_input_type(&mut self, input_type: InputDataRaw) {
    self.fbb_.push_slot::<InputDataRaw>(InputData::VT_INPUT_TYPE, input_type, InputDataRaw::NONE);
  }
  #[inline]
  pub fn add_input(&mut self, input: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(InputData::VT_INPUT, input);
  }
  #[inline]
  pub fn add_distance(&mut self, distance: f32) {
    self.fbb_.push_slot::<f32>(InputData::VT_DISTANCE, distance, 0.0);
  }
  #[inline]
  pub fn add_datamap(&mut self, datamap: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(InputData::VT_DATAMAP, datamap);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> InputDataBuilder<'a, 'b> {
    let start = _fbb.start_table();
    InputDataBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<InputData<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, InputData::VT_UUID,"uuid");
    self.fbb_.required(o, InputData::VT_INPUT,"input");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for InputData<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("InputData");
      ds.field("uuid", &self.uuid());
      ds.field("input_type", &self.input_type());
      match self.input_type() {
        InputDataRaw::Pointer => {
          if let Some(x) = self.input_as_pointer() {
            ds.field("input", &x)
          } else {
            ds.field("input", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InputDataRaw::Hand => {
          if let Some(x) = self.input_as_hand() {
            ds.field("input", &x)
          } else {
            ds.field("input", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("input", &x)
        },
      };
      ds.field("distance", &self.distance());
      ds.field("datamap", &self.datamap());
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_input_data<'a>(buf: &'a [u8]) -> InputData<'a> {
  unsafe { flatbuffers::root_unchecked::<InputData<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_input_data<'a>(buf: &'a [u8]) -> InputData<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<InputData<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `InputData`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_input_data_unchecked`.
pub fn root_as_input_data(buf: &[u8]) -> Result<InputData, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<InputData>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `InputData` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_input_data_unchecked`.
pub fn size_prefixed_root_as_input_data(buf: &[u8]) -> Result<InputData, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<InputData>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `InputData` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_input_data_unchecked`.
pub fn root_as_input_data_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<InputData<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<InputData<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `InputData` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_input_data_unchecked`.
pub fn size_prefixed_root_as_input_data_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<InputData<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<InputData<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a InputData and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `InputData`.
pub unsafe fn root_as_input_data_unchecked(buf: &[u8]) -> InputData {
  flatbuffers::root_unchecked::<InputData>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed InputData and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `InputData`.
pub unsafe fn size_prefixed_root_as_input_data_unchecked(buf: &[u8]) -> InputData {
  flatbuffers::size_prefixed_root_unchecked::<InputData>(buf)
}
#[inline]
pub fn finish_input_data_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<InputData<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_input_data_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<InputData<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod StardustXR
