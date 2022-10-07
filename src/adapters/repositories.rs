use diesel::{SqliteConnection, Insertable};

use super::schema;

pub struct Repository<'a> {
	pub connection: &'a SqliteConnection,
}

impl<'a> Repository<'a> {
    pub fn new(connection: &'a SqliteConnection) -> Self {
		Self { connection }
	}
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
	pub name: &'a str,
	pub last_name: &'a str,
}

#[cfg(test)]
mod tests {
	use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, Connection};
	use super::*;
    use crate::adapters::database::establish_connection;
	use crate::adapters::schema::users::dsl::*;

	#[test]
	fn repository_instance() {
		let connection = establish_connection();
		let repo = Repository::new(&connection);
	}

	#[test]
    fn test_can_create_user() {
		let mut connection = establish_connection();
        let repo = Repository::new(&connection);


		connection.test_transaction::<_, Error, _>(|conn| {
			let new_user = NewUser { name: "Thiago", last_name: "Pacheco"};
				diesel::insert_into(users).values(&new_user).execute(conn).expect("Error");
	
			let results = users
				.select(name)
				.load::<String>(conn)
				.expect("Error loading users");
			assert_eq!(results.len(), 1);		
			Ok(())
		});

	}
}