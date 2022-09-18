use serde::Deserialize;

// Tour database
#[derive(Debug, Deserialize)]
pub struct Database {
    pub tours: Vec<Tour>
}

// Tour information
#[derive(Debug, Deserialize)]
pub struct Tour {
    pub name:       String,
    pub nicknames:  Option<Vec<String>>,
    pub missions:   Vec<Mission>,
}

// Mission information
#[derive(Debug, Deserialize)]
pub struct Mission {
    pub popfile:    String,
    pub name:       String,
    pub nicknames:  Option<Vec<String>>,
}

impl Database {
    pub fn new() -> Self {
        serde_json::from_str(include_str!("data/tours.json")).unwrap()
    }

    pub fn find_tour(&self, name: &String) -> Option<&Tour> {
        for tour in self.tours.iter() {
            if tour == name {
                return Some(tour);
            }
        }
        None
    }

    pub fn find_mission(&self, name: &String) -> Option<&Mission> {
        for tour in self.tours.iter() {
            if let Some(mission) = tour.find_mission(name) {
                return Some(mission);
            }
        }
        None
    }
}

/*
impl Iterator for Database {
    type Item = Tour;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
*/

impl Tour {
    pub fn find_mission(&self, name: &String) -> Option<&Mission> {
        for mission in self.missions.iter() {
            if mission == name {
                return Some(mission);
            }
        }

        None
    }
}

impl PartialEq<String> for Tour {
    fn eq(&self, other: &String) -> bool {
        let other_caseless = unicase::Ascii::new(other);
        // Match name
        if unicase::Ascii::new(&self.name) == other_caseless {
            return true;
        }
        // Match nicknames, if available
        if let Some(nicknames) = &self.nicknames {
            return nicknames.iter().any(|x|
                unicase::Ascii::new(&x) == other_caseless
            );
        }

        false
    }
}

impl PartialEq<String> for Mission {
    fn eq(&self, other: &String) -> bool {
        let other_caseless = unicase::Ascii::new(other);
        // Match popfile
        if unicase::Ascii::new(&self.popfile) == other_caseless {
            return true;
        }
        // Match mission name
        if unicase::Ascii::new(&self.name) == other_caseless {
            return true;
        }
        // Match nicknames, if available
        if let Some(nicknames) = &self.nicknames {
            return nicknames.iter().any(|x|
                unicase::Ascii::new(&x) == other_caseless
            );
        }

        false
    }
}
