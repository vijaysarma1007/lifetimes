fn choose_favorite<'a>(first: &'a str, second: &str) -> &'a str {
    let some_condition = true;
    println!("{second}");
    first
}

fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    println!("{second}");
    first
}

struct DentistAppointment {
    doctor: String,
}

impl DentistAppointment {
    fn book<'a, 'b, 'c>(&'a self, check_in_time: &'b str, check_out_tyime: &'c str) -> &'b str {
        println!(
            "you are booked form {} to {} with doctor {}",
            check_in_time, check_out_tyime, self.doctor
        );
        check_in_time
    }
}

fn main() {
    let appt = DentistAppointment {
        doctor: String::from("David"),
    };
    let result = appt.book("3:00pm", "4:00pm");
    println!("{result:?}");

    let orlando = String::from("Orlando");

    let result = {
        let san_francisco = String::from("San Francisco");
        longest(&orlando, &san_francisco)
    };

    println!("{orlando}");
}
