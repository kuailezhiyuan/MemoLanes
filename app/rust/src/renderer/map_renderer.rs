use crate::api::api::CameraOption;
use crate::journey_area_utils;
use crate::journey_bitmap::JourneyBitmap;

pub struct MapRenderer {
    journey_bitmap: JourneyBitmap,
    changed: bool,
    current_area: Option<u64>,
    // TODO: `provisioned_camera_option` should be moved out and passed to the
    // map separately.
    provisioned_camera_option: Option<CameraOption>,
}

impl MapRenderer {
    pub fn new(journey_bitmap: JourneyBitmap) -> Self {
        Self {
            journey_bitmap,
            changed: false,
            current_area: None,
            provisioned_camera_option: None,
        }
    }

    pub fn set_provisioned_camera_option(&mut self, camera_option: Option<CameraOption>) {
        self.provisioned_camera_option = camera_option;
    }

    pub fn get_provisioned_camera_option(&self) -> Option<CameraOption> {
        self.provisioned_camera_option
    }

    pub fn update<F>(&mut self, f: F)
    where
        F: Fn(&mut JourneyBitmap),
    {
        f(&mut self.journey_bitmap);
        // TODO: we should improve the cache invalidation rule
        self.reset();
    }

    pub fn replace(&mut self, journey_bitmap: JourneyBitmap) {
        self.journey_bitmap = journey_bitmap;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.changed = true;
        self.current_area = None;
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn get_latest_bitmap_if_changed(&mut self) -> Option<&JourneyBitmap> {
        if self.changed {
            self.changed = false;
            Some(&self.journey_bitmap)
        } else {
            None
        }
    }

    pub fn peek_latest_bitmap(&self) -> &JourneyBitmap {
        &self.journey_bitmap
    }

    pub fn get_current_area(&mut self) -> u64 {
        // TODO: we can do something more efficient here, instead of traversing
        // the whole bitmap evey time it changes.
        match self.current_area {
            Some(area) => area,
            None => {
                let journey_bitmap = self.peek_latest_bitmap();
                let area = journey_area_utils::compute_journey_bitmap_area(journey_bitmap);
                self.current_area = Some(area);
                area
            }
        }
    }
}
