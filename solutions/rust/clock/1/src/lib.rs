use std::fmt::{ Display, Formatter, Error };

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter)->Result<(), Error> {
        write!(f, "{:#02}:{:#02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: ( 
                ( (hours%24)+24 ) // find the hour in a day and if its negative converts to positive
                + ( ( (minutes / 60)%24) + 24) // minutes in an hour and if negative converts to +ve
                - ( if minutes < 0 && minutes % 60 != 0 { 1 }else{ 0 } ) // normalize the extra negative minutes
            ) % 24,    //to put all within 24 hours
            minutes: ((minutes % 60) + 60) % 60
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.minutes + minutes;
        if total_minutes > 60 {
            let minutes = total_minutes % 60;
            let hours = self.hours + (total_minutes / 60);
            return Self {
                hours: hours % 24,
                minutes
            };
        }else if total_minutes < 0 {
            let minutes = (total_minutes % 60) + 60; //converts to positive or clockwise reading for negatives
            let hours = (
                ((self.hours + (total_minutes/60) + 24)%24)
                    - ( if total_minutes % 60 != 0 { 1 }else{ 0 } ) + 24 ) 
                % 24;
            return Self {
                hours: hours % 24, 
                minutes
            };
        }

        Self {
            minutes: total_minutes,
            hours: self.hours
        }
    }
}
