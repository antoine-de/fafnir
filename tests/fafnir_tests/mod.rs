use super::mimir;
use super::DATASET;
use super::{ElasticSearchWrapper, PostgresWrapper};
use postgres::Connection;

// Init the Postgres Wrapper
fn init_tests(es_wrapper: &mut ElasticSearchWrapper, pg_wrapper: &PostgresWrapper) {
    let conn = pg_wrapper.get_conn();
    create_tests_tables(&conn);
    populate_tables(&conn);
    load_poi_class_function(&conn);
    load_address(es_wrapper);
}

fn create_tests_tables(conn: &Connection) {
    conn.execute(
        "CREATE TABLE osm_poi_point(
                         id                 serial primary key,
                         osm_id             bigint,
                         name               varchar,
                         name_en            varchar,
                         name_de            varchar,
                         tags               hstore,
                         subclass           varchar,
                         mapping_key        varchar,
                         station            varchar,
                         funicular          varchar,
                         information        varchar,
                         uic_ref            varchar,
                         geometry           geometry,
                         agg_stop           integer
                       )",
        &[],
    ).unwrap();
    conn.execute(
        "CREATE TABLE osm_poi_polygon (
                         id                 serial primary key,
                         osm_id             bigint,
                         name               varchar,
                         name_en            varchar,
                         name_de            varchar,
                         tags               hstore,
                         subclass           varchar,
                         mapping_key        varchar,
                         station            varchar,
                         funicular          varchar,
                         information        varchar,
                         uic_ref            varchar,
                         geometry           geometry
                       )",
        &[],
    ).unwrap();
}

fn populate_tables(conn: &Connection) {
    conn.execute("INSERT INTO osm_poi_point (osm_id, name, name_en, name_de, tags, subclass, mapping_key, station, funicular, information, uic_ref, geometry) VALUES (5589618289, 'Ocean Studio',null,null, '\"name\"=>\"Ocean Studio\", \"amenity\"=>\"cafe\", \"name_int\"=>\"Ocean Studio\", \"name:latin\"=>\"Ocean Studio\"', 'cafe', 'amenity',null,null,null,null, '0101000020110F0000D098707D8D5B6A419AD08C9415704541')", &[]).unwrap();
    conn.execute("INSERT INTO osm_poi_point (osm_id, name, name_en, name_de, tags, subclass, mapping_key, station, funicular, information, uic_ref, geometry) VALUES (5590210422, 'Spagnolo',null,null, '\"name\"=>\"Spagnolo\", \"shop\"=>\"clothes\", \"name_int\"=>\"Spagnolo\", \"name:latin\"=>\"Spagnolo\"', 'clothes', 'shop',null,null,null,null, '0101000020110F0000F33E3B4589031CC1A6CE19ABBB175341')", &[]).unwrap();
    conn.execute("INSERT INTO osm_poi_point (osm_id, name, name_en, name_de, tags, subclass, mapping_key, station, funicular, information, uic_ref, geometry) VALUES (5590601521, '4 gusto',null,null, '\"name\"=>\"4 gusto\", \"amenity\"=>\"cafe\", \"name_int\"=>\"4 gusto\", \"name:latin\"=>\"4 gusto\"', 'cafe', 'amenity',null,null,null,null, '0101000020110F00006091F81AE83E45417DAADADEB2185041')", &[]).unwrap();
    conn.execute("INSERT INTO osm_poi_point (osm_id, name, name_en, name_de, tags, subclass, mapping_key, station, funicular, information, uic_ref, geometry) VALUES (5239101332, 'Le nomade',null,null, '\"name\"=>\"Le nomade\", \"amenity\"=>\"bar\", \"name:es\"=>\"Le nomade\", \"name_int\"=>\"Le nomade\", \"name:latin\"=>\"Le nomade\"', 'bar', 'amenity',null,null,null,null, '0101000020110F00005284822481905EC17327757A8E2C37C1')", &[]).unwrap();
}

/// This function uses the poi_class function from
/// https://github.com/openmaptiles/openmaptiles/blob/master/layers/poi/class.sql
fn load_poi_class_function(conn: &Connection) {
    conn.execute("
            CREATE OR REPLACE FUNCTION poi_class(subclass TEXT, mapping_key TEXT)
            RETURNS TEXT AS $$
                SELECT CASE
                    WHEN subclass IN ('accessories','antiques','beauty','bed','boutique','camera','carpet','charity','chemist','chocolate','coffee','computer','confectionery','convenience','copyshop','cosmetics','garden_centre','doityourself','erotic','electronics','fabric','florist','frozen_food','furniture','video_games','video','general','gift','hardware','hearing_aids','hifi','ice_cream','interior_decoration','jewelry','kiosk','lamps','mall','massage','motorcycle','mobile_phone','newsagent','optician','outdoor','perfumery','perfume','pet','photo','second_hand','shoes','sports','stationery','tailor','tattoo','ticket','tobacco','toys','travel_agency','watches','weapons','wholesale') THEN 'shop'
                    WHEN subclass IN ('townhall','public_building','courthouse','community_centre') THEN 'town_hall'
                    WHEN subclass IN ('golf','golf_course','miniature_golf') THEN 'golf'
                    WHEN subclass IN ('fast_food','food_court') THEN 'fast_food'
                    WHEN subclass IN ('park','bbq') THEN 'park'
                    WHEN subclass IN ('bus_stop','bus_station') THEN 'bus'
                    WHEN (subclass='station' AND mapping_key = 'railway') OR subclass IN ('halt', 'tram_stop', 'subway') THEN 'railway'
                    WHEN (subclass='station' AND mapping_key = 'aerialway') THEN 'aerialway'
                    WHEN subclass IN ('subway_entrance','train_station_entrance') THEN 'entrance'
                    WHEN subclass IN ('camp_site','caravan_site') THEN 'campsite'
                    WHEN subclass IN ('laundry','dry_cleaning') THEN 'laundry'
                    WHEN subclass IN ('supermarket','deli','delicatessen','department_store','greengrocer','marketplace') THEN 'grocery'
                    WHEN subclass IN ('books','library') THEN 'library'
                    WHEN subclass IN ('university','college') THEN 'college'
                    WHEN subclass IN ('hotel','motel','bed_and_breakfast','guest_house','hostel','chalet','alpine_hut','camp_site') THEN 'lodging'
                    WHEN subclass IN ('chocolate','confectionery') THEN 'ice_cream'
                    WHEN subclass IN ('post_box','post_office') THEN 'post'
                    WHEN subclass IN ('cafe') THEN 'cafe'
                    WHEN subclass IN ('school','kindergarten') THEN 'school'
                    WHEN subclass IN ('alcohol','beverages','wine') THEN 'alcohol_shop'
                    WHEN subclass IN ('bar','nightclub') THEN 'bar'
                    WHEN subclass IN ('marina','dock') THEN 'harbor'
                    WHEN subclass IN ('car','car_repair','taxi') THEN 'car'
                    WHEN subclass IN ('hospital','nursing_home', 'clinic') THEN 'hospital'
                    WHEN subclass IN ('grave_yard','cemetery') THEN 'cemetery'
                    WHEN subclass IN ('attraction','viewpoint') THEN 'attraction'
                    WHEN subclass IN ('biergarten','pub') THEN 'beer'
                    WHEN subclass IN ('music','musical_instrument') THEN 'music'
                    WHEN subclass IN ('american_football','stadium','soccer','pitch') THEN 'stadium'
                    WHEN subclass IN ('art','artwork','gallery','arts_centre') THEN 'art_gallery'
                    WHEN subclass IN ('bag','clothes') THEN 'clothing_store'
                    WHEN subclass IN ('swimming_area','swimming') THEN 'swimming'
                    WHEN subclass IN ('castle','ruins') THEN 'castle'
                    ELSE subclass
                END;
            $$ LANGUAGE SQL IMMUTABLE;", &[]).unwrap();
}

fn load_address(es_wrapper: &mut ElasticSearchWrapper) {
    let test_address = get_test_address();
    es_wrapper.make_addr_index(DATASET, &test_address);
}

fn get_test_address() -> mimir::Addr {
    let street = mimir::Street {
        id: "1234".to_string(),
        street_name: "test".to_string(),
        label: "test (ville Test)".to_string(),
        administrative_regions: vec![],
        weight: 50.0,
        zip_codes: vec!["12345".to_string()],
        coord: mimir::Coord::new(124.139607, 24.462216),
    };
    mimir::Addr {
        id: format!("addr:{};{}", 124.139607, 24.462216),
        house_number: "1234".to_string(),
        street: street,
        label: "test (ville Test)".to_string(),
        coord: mimir::Coord::new(124.139607, 24.462216),
        weight: 50.0,
        zip_codes: vec!["12345".to_string()],
    }
}

fn get_label(address: &mimir::Address) -> &str {
    match address {
        &mimir::Address::Street(ref s) => &s.label,
        &mimir::Address::Addr(ref a) => &a.label,
    }
}

fn get_house_number(address: &mimir::Address) -> &str {
    match address {
        &mimir::Address::Street(_) => &"",
        &mimir::Address::Addr(ref a) => &a.house_number,
    }
}

fn get_coord(address: &mimir::Address) -> &mimir::Coord {
    match address {
        &mimir::Address::Street(ref s) => &s.coord,
        &mimir::Address::Addr(ref a) => &a.coord,
    }
}

pub fn main_test(mut es_wrapper: ElasticSearchWrapper, pg_wrapper: PostgresWrapper) {
    init_tests(&mut es_wrapper, &pg_wrapper);
    let fafnir = concat!(env!("OUT_DIR"), "/../../../fafnir");
    super::launch_and_assert(
        fafnir,
        vec![
            format!("--dataset={}", DATASET),
            format!("--es={}", &es_wrapper.host()),
            format!("--pg=postgres://test@{}/test", &pg_wrapper.host()),
        ],
        &es_wrapper,
    );

    // Test that the postgres wrapper contains 4 rows
    let rows = &pg_wrapper.get_rows();
    assert_eq!(rows.len(), 4);

    // Test that the place "Ocean Studio" has been imported in the elastic wrapper
    let pois: Vec<mimir::Place> = es_wrapper
        .search_and_filter("Ocean Studio", |_| true)
        .collect();
    assert_eq!(&pois.len(), &1);

    // Test that the place "Ocean Studio" is a POI
    let ocean_place = &pois[0];
    assert!(&ocean_place.is_poi());

    // Test that the coord property of a POI has been well loaded
    // We test latitude and longitude
    let ocean_poi = &ocean_place.poi().unwrap();
    let coord_ocean_poi = &ocean_poi.coord;
    assert_eq!(&coord_ocean_poi.lat(), &24.46275578041472);
    assert_eq!(&coord_ocean_poi.lon(), &124.13808059594312);

    // Test Label
    let label_ocean_poi = &ocean_poi.label;
    assert_eq!(label_ocean_poi, &"Ocean Studio");

    // Test Properties: the amenity property for this POI should be "cafe"
    let properties_ocean_poi = &ocean_poi.properties;
    let amenity_tag = properties_ocean_poi
        .into_iter()
        .find(|&p| p.key == "amenity")
        .unwrap();
    assert_eq!(amenity_tag.value, "cafe".to_string());

    // Test Address: we get the address from elasticsearch associated to a POI and we check that
    // its associated information are correct.
    // To guarantee the rubber found an address we have put a fake address close to the location of
    // the POI in the init() method.
    let address_ocean_poi = ocean_poi.address.as_ref().unwrap();
    let address_label = get_label(&address_ocean_poi);
    assert_eq!(address_label, &"test (ville Test)".to_string());
    let address_house_number = get_house_number(&address_ocean_poi);
    assert_eq!(address_house_number, "1234".to_string());
    let address_coord = get_coord(&address_ocean_poi);
    assert_eq!(&address_coord.lat(), &24.462216);
    assert_eq!(&address_coord.lon(), &124.139607);
}
