# MongoDB cheat sheet

 - use db_name : create/switch to db_name 
 - show collections
 - db.collection_name.find() equals to select * from collection_name;
 - db.collection_name.insert({content to be added}) equals to insert into collection_name values(...)
 - db.collection_name.drop() equals to drop collection_name
 - db.collection_name.insert or save or update do the same thing via upsert : insert if not exists or update if exists
 - db.collection_name.remove({condigition}) equals to delete from collection_name where condition;
 
