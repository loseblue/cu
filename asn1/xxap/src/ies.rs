// Autogenerated from XXAP-Common-IEs.asn
use asn1_per::{aper::*, *};

// Criticality
#[derive(Clone, Debug, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Criticality {
    Reject,
    Ignore,
    Notify,
}

impl Criticality {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(PerCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_enumerated(data, Some(0), Some(2), false, *self as i128, false)
    }
}

impl PerCodec for Criticality {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Criticality::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("Criticality");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("Criticality");
            e
        })
    }
}
// TransportLayerAddress
#[derive(Clone, Debug)]
pub struct TransportLayerAddress(pub BitString);

impl TransportLayerAddress {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(decode::decode_bitstring(
            data,
            Some(1),
            Some(160),
            true,
        )?))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_bitstring(data, Some(1), Some(160), true, &self.0, false)
    }
}

impl PerCodec for TransportLayerAddress {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        TransportLayerAddress::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("TransportLayerAddress");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("TransportLayerAddress");
            e
        })
    }
}
// GtpTeid
#[derive(Clone, Debug)]
pub struct GtpTeid(pub [u8; 4]);

impl GtpTeid {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(
            decode::decode_octetstring(data, Some(4), Some(4), false)?
                .try_into()
                .unwrap(),
        ))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_octetstring(data, Some(4), Some(4), false, &(self.0).into(), false)
    }
}

impl PerCodec for GtpTeid {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        GtpTeid::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("GtpTeid");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("GtpTeid");
            e
        })
    }
}
// GtpTunnel
#[derive(Clone, Debug)]
pub struct GtpTunnel {
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GtpTeid,
}

impl GtpTunnel {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (optionals, _extensions_present) = decode::decode_sequence_header(data, true, 1)?;
        let transport_layer_address = TransportLayerAddress::decode(data)?;
        let gtp_teid = GtpTeid::decode(data)?;

        // Process the extension container

        if optionals[0] {
            let num_ies = decode::decode_length_determinent(data, Some(1), Some(65535), false)?;
            for _ in 0..num_ies {
                let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _criticality = Criticality::decode(data)?;
                let ie_length = decode::decode_length_determinent(data, None, None, false)?;
                match id {
                    _ => data.advance_maybe_err(ie_length, false)?,
                }
            }
        }
        Ok(Self {
            transport_layer_address,
            gtp_teid,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut optionals = BitString::new();
        optionals.push(false);

        encode::encode_sequence_header(data, true, &optionals, false)?;
        self.transport_layer_address.encode(data)?;
        self.gtp_teid.encode(data)?;

        Ok(())
    }
}

impl PerCodec for GtpTunnel {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        GtpTunnel::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("GtpTunnel");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("GtpTunnel");
            e
        })
    }
}
// PduSessionId
#[derive(Clone, Copy, Debug)]
pub struct PduSessionId(pub u8);

impl PduSessionId {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(
            decode::decode_integer(data, Some(0), Some(255), false)?.0 as u8,
        ))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_integer(data, Some(0), Some(255), false, self.0 as i128, false)
    }
}

impl PerCodec for PduSessionId {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        PduSessionId::decode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("PduSessionId");
            e
        })
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {
            e.push_context("PduSessionId");
            e
        })
    }
}