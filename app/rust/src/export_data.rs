use crate::journey_vector::JourneyVector;
use anyhow::{Ok, Result};
use geo_types::Point;
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};
use kml::{Kml, KmlDocument, KmlWriter};
use std::{
    collections::HashMap,
    io::{Seek, Write},
};

// TODO: Pull in more metadata to the exported files, e.g. timestamp, note, etc
// For most things, we could put them as custom attributes. The timestamp is a
// bit annoying. Ideally I don't want to fake data (e.g. generating timestamps
// for all points based on begin and end time). So maybe also treat them as
// custom attributes or just add timestamp for the first and last point if possible.
pub fn journey_vector_to_gpx_file<T: Write + Seek>(
    journey_vector: &JourneyVector,
    writer: &mut T,
) -> Result<()> {
    let mut segments = Vec::new();
    // Add track point
    for track_segment in &journey_vector.track_segments {
        let mut points = Vec::new();
        track_segment.track_points.iter().for_each(|point| {
            points.push(Waypoint::new(Point::new(point.longitude, point.latitude)));
        });
        segments.push(TrackSegment { points });
    }
    let track = Track {
        name: Some("Track 1".to_string()),
        comment: None,
        description: None,
        source: None,
        links: vec![],
        type_: None,
        number: None,
        segments,
    };
    let gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: None,
        metadata: None,
        waypoints: vec![],
        tracks: vec![track],
        routes: vec![],
    };
    gpx::write(&gpx, writer)?;
    Ok(())
}

pub fn journey_vector_to_kml_file<T: Write + Seek>(
    journey_vector: &JourneyVector,
    writer: &mut T,
) -> Result<()> {
    let style = kml::types::Style {
        ..kml::types::Style::default()
    };

    let mut elements = vec![Kml::Style(style)];

    for track_segment in &journey_vector.track_segments {
        let mut coords = Vec::new();
        let mut gx_coords = Vec::new();
        track_segment.track_points.iter().for_each(|point| {
            coords.push(kml::types::Coord {
                x: point.longitude,
                y: point.latitude,
                z: None,
            });
            gx_coords.push(kml::types::Element {
                name: "gx:coord".to_owned(),
                content: Some(format!("{} {} {}", point.longitude, point.latitude, 0)),
                ..kml::types::Element::default()
            })
        });
        let geometry = kml::types::LineString {
            coords,
            tessellate: true,
            ..kml::types::LineString::default()
        };

        let placemark = kml::types::Placemark {
            name: Some("export".to_string()),
            children: vec![kml::types::Element {
                name: "gx:Track".to_owned(),
                content: None,
                children: gx_coords,
                ..kml::types::Element::default()
            }],
            geometry: Some(kml::types::Geometry::LineString(geometry)),
            ..kml::types::Placemark::default()
        };
        elements.push(kml::Kml::Placemark(placemark))
    }

    write_kml_document(
        "memolanes".to_owned(),
        "Generated by memolanes".to_owned(),
        elements,
        writer,
    )?;
    Ok(())
}

fn write_kml_document<T: Write + Seek>(
    name: String,
    description: String,
    elements: Vec<Kml>,
    writer: &mut T,
) -> Result<()> {
    let document = KmlDocument::<f64> {
        version: kml::KmlVersion::V22,
        attrs: HashMap::from([
            (
                "xmlns".to_owned(),
                "http://www.opengis.net/kml/2.2".to_owned(),
            ),
            (
                "xmlns:gx".to_owned(),
                "http://www.google.com/kml/ext/2.2".to_owned(),
            ),
            (
                "xmlns:kml".to_owned(),
                "http://www.opengis.net/kml/2.2".to_owned(),
            ),
            (
                "xmlns:atom".to_owned(),
                "http://www.w3.org/2005/Atom".to_owned(),
            ),
        ]),
        elements: vec![Kml::Folder {
            attrs: HashMap::from([
                ("name".to_owned(), name),
                ("description".to_owned(), description),
            ]),
            elements,
        }],
    };

    let mut writer = KmlWriter::<_, f64>::from_writer(writer);
    let kml = Kml::KmlDocument(document);
    writer.write(&kml)?;
    Ok(())
}
