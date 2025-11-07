use anyhow::Result;

use crate::mock::*;
use todo::domain::TodoListRepository;
// fn create_table(&self) -> Result<()>;

#[test]
fn add() -> Result<()> {
    let mock_env = MockListEnv::new()?;
    let repo = mock_env.repo();

    repo.add("new_list")?;

    let count: i64 = count_entries(&mock_env.db.conn, "collection")?;
    assert_eq!(count, 2); // by default MockListEnv adds the list 'todos'

    Ok(())
}

#[test]
fn fetch_all() -> Result<()> {
    let mock_env = MockListEnv::new()?;
    let repo = mock_env.repo();
    repo.add("new_list")?;

    let lists = repo.fetch_all()?;
    assert_eq!(lists.len(), 2); // by default MockListEnv adds the list 'todos'

    Ok(())
}

#[test]
fn fetch_id() -> Result<()> {
    let mock_env = MockListEnv::new()?;
    let repo = mock_env.repo();
    repo.add("new_list")?;

    let new_list_id = repo.fetch_id("new_list")?;
    assert_eq!(new_list_id, 2); // by default MockListEnv adds the list 'todos'

    Ok(())
}

#[test]
fn delete() -> Result<()> {
    let mock_env = MockListEnv::new()?;
    let repo = mock_env.repo();
    repo.add("new_list")?;
    let count: i64 = count_entries(&mock_env.db.conn, "collection")?;
    assert_eq!(count, 2); // by default MockListEnv adds the list 'todos'

    repo.delete("new_list")?;
    let count: i64 = count_entries(&mock_env.db.conn, "collection")?;
    assert_eq!(count, 1); // by default MockListEnv adds the list 'todos'

    Ok(())
}
