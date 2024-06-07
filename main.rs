// Tugas akhir pelatihan KMTETI
// Nama : Christiano Neil Imantaka
// Prodi : Teknik Nuklir

use std::io;
use std::collections::BTreeMap;
use std::cmp;

// Struct untuk profil pengguna
// "item_count" untuk menunjukkan jumlah barang di suatu profil
struct Profile
{
  username : String,
  item_count : usize,
}

// Implementasi untuk struct "Profile"
impl Profile
{
  fn get_username(&mut self) -> &str
  {
    return &self.username;
  }

  fn get_item_count(&mut self) -> usize
  {
    return self.item_count;

  }

  fn update_item_count(&mut self, new_count : usize)
  {
    self.item_count = new_count;
  }
}

// Fungsi untuk menampilkan isi inventory dalam bentuk tabel dan mengembalikan nilai boolean untuk mengindikasikan apakah inventory kosong
fn show_inventory(items_inventory : &BTreeMap::<String, u64>) -> bool
{
  // Nama dari isi tabel
  let num_str : &str = "Nomor";
  let name_str : &str = "Nama barang";
  let quantity_str : &str = "Jumlah barang";

  // Menghitung nama terpanjang
  let longest_name : usize =
  {
    let mut name_length : usize = 0;

    for name in items_inventory.keys()
    {
      name_length = cmp::max( name.len(), name_length );
    } 

    name_length
  };

  // Menghitung kuantitas terpanjang dalam string
  let longest_quantity =
  {
    let mut quantity_length : usize = 0;

    for quantity in items_inventory.values()
    {
      quantity_length = cmp::max( quantity.to_string().len(), quantity_length );
    } 

    quantity_length
  };

  // Hitung lebar untuk penomoran
  let num_width : usize =
  match num_str.len() >= items_inventory.len().to_string().len()
  {
    true => num_str.len() + 2,
    false => items_inventory.len().to_string().len() + 2,
  };

  // Hitung lebar untuk nama barang
  let name_width : usize =
  match name_str.len() >= longest_name
  {
    true => name_str.len() + 2,
    false => longest_name + 2,
  };

  // Hitung lebar untuk kuantitas
  let quantity_width : usize =
  match quantity_str.len() >= longest_quantity
  {
    true => quantity_str.len() + 2,
    false => longest_quantity + 2,
  };
  
  // Menampilkan tabel
  println!("+{:-^width1$}+{:-^width2$}+{:-^width3$}+", "", "", "", width1 = num_width, width2 = name_width, width3 = quantity_width);
  println!("|{:^width1$}|{:^width2$}|{:^width3$}|", num_str, name_str, quantity_str, width1 = num_width, width2 = name_width, width3 = quantity_width);
  println!("+{:-^width1$}+{:-^width2$}+{:-^width3$}+", "", "", "", width1 = num_width, width2 = name_width, width3 = quantity_width);
  
  for ( (item_name, item_quantity), number) in items_inventory.into_iter().zip( 1..=items_inventory.len() )
  {
    println!("|{:<width1$}|{:<width2$}|{:<width3$}|", number, item_name, item_quantity, width1 = num_width, width2 = name_width, width3 = quantity_width);
  }

  println!("+{:-^width1$}+{:-^width2$}+{:-^width3$}+", "", "", "", width1 = num_width, width2 = name_width, width3 = quantity_width);

  return items_inventory.len() != 0;
}

// Fungsi untuk menambah barang
fn add_item(items_inventory : &mut BTreeMap::<String, u64>)
{
  let mut input_item_name : String = String::new(); // Input untuk nama item
  let mut input_item_quantity : String = String::new(); // Input untuk kuantitas item

  println!("Silakan masukkan nama barang : ");
  io::stdin().read_line(&mut input_item_name).unwrap();
  input_item_name = input_item_name.trim().to_string();

  println!("\nSilakan masukkan kuantitas barang : ");
  io::stdin().read_line(&mut input_item_quantity).unwrap();
  input_item_quantity = input_item_quantity.trim().to_string();

  match input_item_quantity.parse::<u64>()
  {
    Ok(quantity) =>
    {
      items_inventory.entry( input_item_name.clone() ).or_insert( quantity );
      println!("\nBarang berhasil ditambah!\n");
    },
    Err(_) => println!("Input tidak valid!"),
  }
}

// Fungsi untuk menghapus barang
fn delete_item(items_inventory : &mut BTreeMap::<String, u64>)
{
  let inventory_status : bool = show_inventory(items_inventory); // Apakah ada barang di inventory
  let mut input_item_number : String = String::new(); // Input nomor barang (dimulai dari 1) sesuai tabel yang ingin dihapus
  let index : usize; // Index dari barang yang ingin dihapus
  let to_delete : Vec<String> = Vec::from(items_inventory.keys().cloned().collect::<Vec<_>>());

  match inventory_status
  {
    true =>
    {
      println!("\nSilakan pilih barang yang akan dihapus : ");
      io::stdin().read_line(&mut input_item_number).unwrap();
      input_item_number = input_item_number.trim().to_string();

      match input_item_number.parse::<usize>()
      {
        Ok(number) => {index = number-1;},
        Err(_) =>
        {
          println!("Input tidak valid!\n");
          return ();
        },
      }

    },

    false =>
    {
      println!("\nTidak ada barang yang dapat dihapus.\n");
      return ();
    },
  }

  match (items_inventory.len() - 1) >= index
  {
    true => {items_inventory.remove(&to_delete[index].clone()); println!("Barang berhasil dihapus!\n");},
    false => println!("Nomor melebihi jumlah barang yang ada!\n"),
  }
}


// Fungsi untuk mengubah kuantitas barang
fn modify_quantity(items_inventory : &mut BTreeMap::<String, u64>)
{
  let inventory_status : bool = show_inventory(items_inventory); // Apakah ada barang di inventory
  let mut input_item_number : String = String::new(); // Input nomor barang (dimulai dari 1) sesuai tabel yang ingin diubah
  let mut input_item_quantity : String = String::new(); // Input kuantitas baru barang
  let index : usize; // Index dari barang yang ingin diubah
  let to_modify : Vec<String> = Vec::from(items_inventory.keys().cloned().collect::<Vec<_>>());
  
  match inventory_status
  {
    true =>
    {
      println!("\nSilakan pilih barang yang akan diubah kuantitasnya : ");
      io::stdin().read_line(&mut input_item_number).unwrap();
      input_item_number = input_item_number.trim().to_string();

      match input_item_number.parse::<usize>()
      {
        Ok(number) => {index = number-1;},
        Err(_) =>
        {
          println!("Input tidak valid!\n");
          return ();
        },
      }
    },

    false =>
    {
      println!("\nTidak ada barang yang dapat diubah kuantitasnya.\n");
      return ();
    },
  }

  match (items_inventory.len() - 1) >= index
  {
    true =>
    {
      println!("\nSilakan masukkan kuantitas baru : ");
      io::stdin().read_line(&mut input_item_quantity).unwrap();
      input_item_quantity = input_item_quantity.trim().to_string();

      match input_item_quantity.parse::<u64>()
      {
        Ok(new_quantity) => {items_inventory.insert(to_modify[index].clone(), new_quantity); println!("kuantitas berhasil diubah!\n");},
        Err(_) => println!("Input tidak valid!\n"),
      }
    },

    false => println!("Nomor melebihi jumlah barang yang ada!\n"),
  }
}

// Fungsi untuk mengurangi kuantitas barang
fn subtract_quantity(items_inventory : &mut BTreeMap::<String, u64>)
{
  let inventory_status : bool = show_inventory(items_inventory); // Apakah ada barang di inventory
  let mut input_item_number : String = String::new(); // Input nomor barang (dimulai dari 1) sesuai tabel yang ingin dikurangi
  let mut input_subtracted_item : String = String::new(); // Input berapa banyak kuantitas barang yang akan dikurangi
  let index : usize; // Index dari barang yang ingin dikurangi
  let to_modify : Vec<String> = Vec::from(items_inventory.keys().cloned().collect::<Vec<_>>());
  
  match inventory_status
  {
    true =>
    {
      println!("\nSilakan pilih barang yang akan dikurangi kuantitasnya : ");
      io::stdin().read_line(&mut input_item_number).unwrap();
      input_item_number = input_item_number.trim().to_string();

      match input_item_number.parse::<usize>()
      {
        Ok(number) => {index = number-1;},
        Err(_) =>
        {
          println!("Input tidak valid!\n");
          return ();
        },
      }
    },

    false =>
    {
      println!("\nTidak ada barang yang dapat dikurangi kuantitasnya.\n");
      return ();
    },
  }

  match (items_inventory.len() - 1) >= index
  {
    true =>
    {
      println!("\nSilakan masukkan berapa banyak yang akan dihapus : ");
      io::stdin().read_line(&mut input_subtracted_item).unwrap();
      input_subtracted_item = input_subtracted_item.trim().to_string();
  
      match input_subtracted_item.parse::<u64>()
      {
        Ok(subtracted_quantity) =>
        {
          items_inventory.insert(to_modify[index].clone(), items_inventory[&to_modify[index].clone()] - subtracted_quantity);
          println!("Kuantitas berhasil dikurangi!\n");
        },
        Err(_) => println!("Input tidak valid!\n"),
      }
    },

    false => println!("Nomor melebihi jumlah barang yang ada!\n"),
  }
}

// Fungsi untuk menambah kuantitas barang
fn add_quantity(items_inventory : &mut BTreeMap::<String, u64>)
{
  let inventory_status : bool = show_inventory(items_inventory); // Apakah ada barang di inventory
  let mut input_item_number : String = String::new(); // Input nomor barang (dimulai dari 1) sesuai tabel yang ingin ditambah
  let mut input_added_item : String = String::new(); // Input berapa banyak kuantitas barang yang akan ditambah
  let index : usize; // Index dari barang yang ingin ditambah
  let to_modify : Vec<String> = Vec::from(items_inventory.keys().cloned().collect::<Vec<_>>());
  
  match inventory_status
  {
    true =>
    {
      println!("\nSilakan pilih barang yang akan ditambah kuantitasnya : ");
      io::stdin().read_line(&mut input_item_number).unwrap();
      input_item_number = input_item_number.trim().to_string();

      match input_item_number.parse::<usize>()
      {
        Ok(number) => {index = number-1;},
        Err(_) =>
        {
          println!("Input tidak valid!\n");
          return ();
        },
      }
    },

    false =>
    {
      println!("\nTidak ada barang yang dapat ditambah kuantitasnya.\n");
      return ();
    },
  }

  match (items_inventory.len() - 1) >= index
  {
    true =>
    {
      println!("\nSilakan masukkan berapa banyak yang akan ditambah : ");
      io::stdin().read_line(&mut input_added_item).unwrap();
      input_added_item = input_added_item.trim().to_string();
  
      match input_added_item.parse::<u64>()
      {
        Ok(added_quantity) =>
        {
          items_inventory.insert(to_modify[index].clone(), items_inventory[&to_modify[index].clone()] - added_quantity);
          println!("Kuantitas berhasil ditambah!\n");
        },
        Err(_) => println!("Input tidak valid!\n"),
      }
    },

    false => println!("Nomor melebihi jumlah barang yang ada!\n"),
  }
}

// Fungsi untuk menampilkan pilihan
fn print_choice_list()
{
  println!("========== Manajemen Gudang ==========");
  println!("1. Tambah barang baru");
  println!("2. Hapus barang");
  println!("3. Ubah kuantitas barang");
  println!("4. Kurangi kuantitas barang");
  println!("5. Tambah kuantitas barang");
  println!("6. Cek gudang");
  println!("7. Tutup aplikasi");
  println!("======================================");
}

fn inventory_menu(profile : &mut Profile, items_inventory : &mut BTreeMap::<String, u64>)
{
  let mut input_choice : String = String::new(); // String untuk menerima input opsi

  println!("Selamat datang, {}!\n", profile.get_username());

  loop
  {
    print_choice_list();
    println!("Jumlah barang : {}", profile.get_item_count());
    println!("\nSilakan pilih aksi :");
    io::stdin().read_line( &mut input_choice ).unwrap();
    input_choice = String::from( input_choice.trim() );

    println!("");

    match input_choice.as_str()
    {
      "1" =>
      {
        println!("========== Menambah barang ==========\n");
        add_item(items_inventory);
      },

      "2" =>
      {
        println!("========== Menghapus barang ==========\n");
        delete_item(items_inventory);
      },

      "3" =>
      {
        println!("========== Mengubah kuantitas barang ==========\n");
        modify_quantity(items_inventory);
      },

      "4" =>
      {
        println!("========== Mengurangi kuantitas barang ==========\n");
        subtract_quantity(items_inventory);
      },

      "5" =>
      {
        println!("========== Menambah kuantitas barang ==========\n");
        add_quantity(items_inventory);
      },

      "6" =>
      {
        println!("========== Menampilkan barang ==========\n");
        show_inventory(items_inventory);
        println!("");
      },


      "7" => 
      {
        println!("Terima kasih telah menggunakan aplikasi ini, sampai jumpa!");
        break;
      },
       _  => println!("Input tidak valid!"),
    }

    profile.update_item_count(items_inventory.len());
    input_choice.clear();

  }

}

fn main()
{
  let mut username : String = String::new();
  let mut items_inventory : BTreeMap<String, u64> = BTreeMap::new();

  println!("Masukkan nama anda : ");
  io::stdin().read_line(&mut username).unwrap();
  username = String::from( username.trim() );
  let mut profile : Profile = Profile{username : username, item_count : 0};

  inventory_menu(&mut profile, &mut items_inventory);

}
