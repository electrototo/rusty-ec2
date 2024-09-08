use aws_sdk_ec2::types::Reservation;

pub struct App {
    pub reservations: Vec<LocalReservation>,
    pub exit: bool,
    pub selected_instance: usize,
    pub aws_profile_name: String
}

pub struct LocalReservation {
    pub reservation: Reservation,
    pub selected: bool
}