use uuid::Uuid;
use actix_web::web::{Json, Path};
use actix_web::{post, get, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


use crate::response::Response;
use crate::constants::APPLICATION_JSON;

pub type Students = Response<Student>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Student {
	pub id: String,
	pub created_at:  DateTime<Utc>,
	pub name: String,
}

impl Student{
	pub fn new(name: String) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			created_at: Utc::now(),
			name
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StudentRequest{
	pub name: Option<String>
}

impl StudentRequest{
	pub fn to_student(&self) -> Option<Student>{
		match &self.name{
			Some(name) => Some(Student::new(name.to_string())),
			None => None
		}
	}
}

#[get("/students")]
pub async fn all() -> HttpResponse {
	let mut students: Vec<Student> = Vec::new();
	students.push(Student::new("pesho".to_string()));
	students.push(Student::new("gosho".to_string()));

	return HttpResponse::Ok()
		.content_type(APPLICATION_JSON)
		.json(students);

}

#[post("/student")]
pub async fn create(student_request: Json<StudentRequest>) -> HttpResponse{
	return HttpResponse::Created()
		.content_type(APPLICATION_JSON)
		.json(student_request.to_student())
}

#[get("/student")]
pub async fn get() -> HttpResponse {
    let name = String::from("Ivan Minkov");
	let student = Student::new(name);

	return HttpResponse::Ok()
		.content_type(APPLICATION_JSON)
		.json(student)
}
