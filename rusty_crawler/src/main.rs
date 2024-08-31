use reqwest;
use scraper;
use csv;

//
// 
struct Fullprofessors {
    staff_name: String, //Option<String>, // link to profiles
    // image: Option<String>,
    //position: Option<String>,
    view_profile:  Option<String>,//String, // link to profiles
    //highest_qualification: String,
    //interest_areas: String,
    //view_profile: String
}
//

fn main() {
    // vec to store data
    let mut list_full_staff: Vec<Fullprofessors> = Vec::new();

    // request sends HTTP GET to passed url parameter
    // I read in the html
    let response = reqwest::blocking::get("https://staff.tukenya.ac.ke/index.php?r=portal/profile/list");

    // get the html content
    let html_content = response.unwrap().text().unwrap();
    //println!("{:?}", html_content);
    
    // parse content
    let documen_t = scraper::Html::parse_document(&html_content);
    // println!("{:?}", documen_t);
    
    // all spans ie I want to get the #content > a:n-th child elements
    // create selector then select
    let html_div_content = scraper::Selector::parse("#content > a").unwrap();
    let html_a_h_content = documen_t.select(&html_div_content);
    //
    for html_position in html_a_h_content {
        // for each n-th child

        // name from the h5 within the nth child
        // there is a selector to capture the h5 then get the text string with names
        let staff_name = html_position.text().collect::<String>();
                //.select(&scraper::Selector::parse("h5.link").unwrap())
                //.next()//.expect("NO").value();
                //.map(|b| b.text().collect::<String>());
                
        // we already selected the element, which is within the a so now we take the n-th child href
        let view_profile = html_position.value().attr("href").map(str::to_string);// convert Option<&str> to Option<String>?;    
        
        // println!("{:?}", staff_name);
        //println
        // instantiate new profile
        let staff_name_profile = Fullprofessors {
                    staff_name,
                    view_profile,
            };
            
        // push to list
        list_full_staff.push(staff_name_profile);          
    }

    // create csv output
    let path = std::path::Path::new("full_professors.csv"); 
    let mut writer = csv::Writer::from_path(path).unwrap();
    
    writer
        .write_record(&["staff_name", "link_profile"])
        .unwrap();

    // populate output file
    for staff_list in list_full_staff {
        let staff_name = staff_list.staff_name;
        let link_profile = staff_list.view_profile;
        
        writer.write_record(&[staff_name, link_profile.expect("REAS")]).unwrap();
     }
    writer.flush().unwrap();
}
