use aws_sdk_ec2::types::Reservation;

pub struct App {
    pub reservations: Vec<LocalReservation>,
    pub exit: bool,
    pub selected_instance: usize
}

pub struct LocalReservation {
    pub reservation: Reservation,
    pub selected: bool
}