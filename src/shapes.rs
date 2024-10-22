use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Serialize};

pub struct SerializableMultiPolygon(pub geo::MultiPolygon);

impl Serialize for SerializableMultiPolygon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0 .0.len()))?;
        for poly in &self.0 .0 {
            seq.serialize_element(&SerializablePolygon(poly.clone()))?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SerializableMultiPolygon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(MultiPolygonVisitor)
    }
}

struct MultiPolygonVisitor;

impl<'de> Visitor<'de> for MultiPolygonVisitor {
    type Value = SerializableMultiPolygon;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a list of polygons")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut polygons = Vec::new();
        while let Some(poly) = seq.next_element::<SerializablePolygon>()? {
            polygons.push(poly.0);
        }
        Ok(SerializableMultiPolygon(geo::MultiPolygon(polygons)))
    }
}

struct SerializablePolygon(pub geo::Polygon<f64>);

impl Serialize for SerializablePolygon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1 + self.0.interiors().len()))?;
        seq.serialize_element(&SerializableLineString(self.0.exterior().clone()))?;

        for interior in self.0.interiors() {
            seq.serialize_element(&SerializableLineString(interior.clone()))?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for SerializablePolygon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(PolygonVisitor)
    }
}

struct PolygonVisitor;

impl<'de> Visitor<'de> for PolygonVisitor {
    type Value = SerializablePolygon;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a list of polygons")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut exterior = None;
        let mut interiors = Vec::new();

        while let Some(line) = seq.next_element::<SerializableLineString>()? {
            if exterior.is_none() {
                exterior = Some(line.0);
            } else {
                interiors.push(line.0);
            }
        }

        Ok(SerializablePolygon(geo::Polygon::new(
            exterior.unwrap(),
            interiors,
        )))
    }
}

struct SerializableLineString(pub geo::LineString<f64>);

impl Serialize for SerializableLineString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0 .0.len()))?;
        for c in &self.0 .0 {
            seq.serialize_element(&[c.x, c.y])?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SerializableLineString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(LineStringVisitor)
    }
}

struct LineStringVisitor;

impl<'de> Visitor<'de> for LineStringVisitor {
    type Value = SerializableLineString;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a list of coordinates")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut coords = Vec::new();
        while let Some(c) = seq.next_element::<[f64; 2]>()? {
            coords.push(geo::Coord { x: c[0], y: c[1] });
        }
        Ok(SerializableLineString(geo::LineString(coords)))
    }
}
