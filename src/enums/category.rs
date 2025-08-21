//! Enum to represent Google Trend Categories.

use serde::Serialize;

/// Represent Google Trend Categories.
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    #[serde(rename = "642")]
    ADDAndADHD,

    #[serde(rename = "625")]
    AIDSAndHIV,

    #[serde(rename = "1289")]
    AcademicConferencesAndPublications,

    #[serde(rename = "427")]
    AccidentAndPersonalInjuryLaw,

    #[serde(rename = "278")]
    AccountingAndAuditing,

    #[serde(rename = "1341")]
    AccountingAndFinancialSoftware,

    #[serde(rename = "894")]
    ActingAndTheater,

    #[serde(rename = "1097")]
    ActionAndAdventureFilms,

    #[serde(rename = "1311")]
    ActionAndPlatformGames,

    #[serde(rename = "1239")]
    AcupunctureAndChineseMedicine,

    #[serde(rename = "820")]
    Acura,

    #[serde(rename = "974")]
    Adoption,

    #[serde(rename = "925")]
    AdventureGames,

    #[serde(rename = "707")]
    AdventureTravel,

    #[serde(rename = "25")]
    AdvertisingAndMarketing,

    #[serde(rename = "356")]
    AerospaceAndDefense,

    #[serde(rename = "326")]
    AffiliatePrograms,

    #[serde(rename = "547")]
    AfricanAmericans,

    #[serde(rename = "1208")]
    AfricanMusic,

    #[serde(rename = "579")]
    AfricansAndDiaspora,

    #[serde(rename = "623")]
    AgingAndGeriatrics,

    #[serde(rename = "748")]
    AgriculturalEquipment,

    #[serde(rename = "46")]
    AgricultureAndForestry,

    #[serde(rename = "1389")]
    Agritourism,

    #[serde(rename = "670")]
    Agrochemicals,

    #[serde(rename = "1247")]
    AirForce,

    #[serde(rename = "203")]
    AirTravel,

    #[serde(rename = "1245")]
    AirportParkingAndTransportation,

    #[serde(rename = "277")]
    AlcoholicBeverages,

    #[serde(rename = "0")]
    All,

    #[serde(rename = "626")]
    Allergies,

    #[serde(rename = "499")]
    AlternativeAndNaturalMedicine,

    #[serde(rename = "1015")]
    AlumniAndReunions,

    #[serde(rename = "624")]
    AlzheimerDisease,

    #[serde(rename = "258")]
    AmericanFootball,

    #[serde(rename = "788")]
    Anatomy,

    #[serde(rename = "400")]
    AncestryAndGenealogy,

    #[serde(rename = "882")]
    AnimalProductsAndServices,

    #[serde(rename = "883")]
    AnimalWelfare,

    #[serde(rename = "1104")]
    AnimatedFilms,

    #[serde(rename = "317")]
    AnimeAndManga,

    #[serde(rename = "64")]
    AntiquesAndCollectibles,

    #[serde(rename = "315")]
    AntivirusAndMalware,

    #[serde(rename = "639")]
    AnxietyAndStress,

    #[serde(rename = "378")]
    ApartmentsAndResidentialRentals,

    #[serde(rename = "68")]
    Apparel,

    #[serde(rename = "1228")]
    ApparelServices,

    #[serde(rename = "747")]
    Aquaculture,

    #[serde(rename = "1034")]
    ArabAndMiddleEasternMusic,

    #[serde(rename = "556")]
    ArabsAndMiddleEasterners,

    #[serde(rename = "919")]
    ArcadeAndCoinOpGames,

    #[serde(rename = "477")]
    Architecture,

    #[serde(rename = "1248")]
    Army,

    #[serde(rename = "1361")]
    ArtAndCraftSupplies,

    #[serde(rename = "628")]
    Arthritis,

    #[serde(rename = "3")]
    ArtsAndEntertainment,

    #[serde(rename = "1195")]
    ArtsEducation,

    #[serde(rename = "912")]
    AsianCuisine,

    #[serde(rename = "1257")]
    AsiansAndDiaspora,

    #[serde(rename = "649")]
    AssistedLivingAndLongTermCare,

    #[serde(rename = "1352")]
    AssistiveTechnology,

    #[serde(rename = "627")]
    Asthma,

    #[serde(rename = "448")]
    AstrologyAndDivination,

    #[serde(rename = "435")]
    Astronomy,

    #[serde(rename = "983")]
    AthleticApparel,

    #[serde(rename = "1254")]
    AtmosphericScience,

    #[serde(rename = "292")]
    Auctions,

    #[serde(rename = "821")]
    Audi,

    #[serde(rename = "1089")]
    AudioAndMusicSoftware,

    #[serde(rename = "361")]
    AudioEquipment,

    #[serde(rename = "1092")]
    AudioFilesFormatsAndCodecs,

    #[serde(rename = "1217")]
    AutoExterior,

    #[serde(rename = "468")]
    AutoFinancing,

    #[serde(rename = "467")]
    AutoInsurance,

    #[serde(rename = "1218")]
    AutoInterior,

    #[serde(rename = "1190")]
    AutomotiveIndustry,

    #[serde(rename = "47")]
    AutosAndVehicles,

    #[serde(rename = "662")]
    Aviation,

    #[serde(rename = "822")]
    BMW,

    #[serde(rename = "1374")]
    BabiesAndToddlers,

    #[serde(rename = "1231")]
    BabyAndPetNames,

    #[serde(rename = "115")]
    BabyCareAndHygiene,

    #[serde(rename = "907")]
    BakedGoods,

    #[serde(rename = "37")]
    Banking,

    #[serde(rename = "423")]
    Bankruptcy,

    #[serde(rename = "259")]
    Baseball,

    #[serde(rename = "264")]
    Basketball,

    #[serde(rename = "1365")]
    Bathroom,

    #[serde(rename = "1074")]
    BeachesAndIslands,

    #[serde(rename = "44")]
    BeautyAndFitness,

    #[serde(rename = "1219")]
    BeautyPageants,

    #[serde(rename = "948")]
    BedAndBath,

    #[serde(rename = "1369")]
    BeddingAndBedLinens,

    #[serde(rename = "1366")]
    Bedroom,

    #[serde(rename = "1367")]
    BedsAndHeadboards,

    #[serde(rename = "404")]
    Beer,

    #[serde(rename = "1059")]
    Bentley,

    #[serde(rename = "1191")]
    BicyclesAndAccessories,

    #[serde(rename = "939")]
    Billiards,

    #[serde(rename = "1384")]
    BinocularsTelescopesAndOpticalDevices,

    #[serde(rename = "690")]
    BiographiesAndQuotations,

    #[serde(rename = "440")]
    BiologicalSciences,

    #[serde(rename = "884")]
    Birds,

    #[serde(rename = "198")]
    BirthControl,

    #[serde(rename = "1270")]
    BirthdaysAndNameDays,

    #[serde(rename = "504")]
    BloggingResourcesAndServices,

    #[serde(rename = "1394")]
    BluRayPlayersAndRecorders,

    #[serde(rename = "1040")]
    Blues,

    #[serde(rename = "1170")]
    BluetoothAccessories,

    #[serde(rename = "920")]
    BoardGames,

    #[serde(rename = "459")]
    Boating,

    #[serde(rename = "1140")]
    BoatsAndWatercraft,

    #[serde(rename = "239")]
    BodyArt,

    #[serde(rename = "241")]
    Bodybuilding,

    #[serde(rename = "360")]
    BollywoodAndSouthAsianFilm,

    #[serde(rename = "355")]
    BookRetailers,

    #[serde(rename = "22")]
    BooksAndLiterature,

    #[serde(rename = "1016")]
    Bowling,

    #[serde(rename = "515")]
    Boxing,

    #[serde(rename = "1287")]
    BrazilianMusic,

    #[serde(rename = "112")]
    BroadcastAndNetworkNews,

    #[serde(rename = "1243")]
    BroadwayAndMusicalTheater,

    #[serde(rename = "862")]
    Buddhism,

    #[serde(rename = "1060")]
    Buick,

    #[serde(rename = "650")]
    BuildingMaterialsAndSupplies,

    #[serde(rename = "708")]
    BusAndRail,

    #[serde(rename = "1272")]
    BusinessAndCorporateLaw,

    #[serde(rename = "12")]
    BusinessAndIndustrial,

    #[serde(rename = "377")]
    BusinessAndPersonalListings,

    #[serde(rename = "498")]
    BusinessAndProductivitySoftware,

    #[serde(rename = "1375")]
    BusinessCardsAndStationary,

    #[serde(rename = "799")]
    BusinessEducation,

    #[serde(rename = "1138")]
    BusinessFinance,

    #[serde(rename = "1200")]
    BusinessFormation,

    #[serde(rename = "784")]
    BusinessNews,

    #[serde(rename = "1159")]
    BusinessOperations,

    #[serde(rename = "336")]
    BusinessPlansAndPresentations,

    #[serde(rename = "721")]
    BusinessProcess,

    #[serde(rename = "329")]
    BusinessServices,

    #[serde(rename = "1300")]
    CADAndCAM,

    #[serde(rename = "731")]
    CAndCPlusPlus,

    #[serde(rename = "217")]
    CDAndAudioShopping,

    #[serde(rename = "1321")]
    CDAndDVDDrivesAndBurners,

    #[serde(rename = "1322")]
    CDAndDVDStorageMedia,

    #[serde(rename = "501")]
    CableAndSatelliteProviders,

    #[serde(rename = "823")]
    Cadillac,

    #[serde(rename = "691")]
    CalculatorsAndReferenceTools,

    #[serde(rename = "1358")]
    CalendarAndSchedulingSoftware,

    #[serde(rename = "389")]
    CallingCards,

    #[serde(rename = "308")]
    Camcorders,

    #[serde(rename = "573")]
    CameraAndPhotoEquipment,

    #[serde(rename = "1383")]
    CameraLenses,

    #[serde(rename = "307")]
    Cameras,

    #[serde(rename = "306")]
    CamerasAndCamcorders,

    #[serde(rename = "398")]
    CampaignsAndElections,

    #[serde(rename = "1213")]
    CampersAndRVs,

    #[serde(rename = "429")]
    Cancer,

    #[serde(rename = "906")]
    CandyAndSweets,

    #[serde(rename = "230")]
    CarAudio,

    #[serde(rename = "1188")]
    CarElectronics,

    #[serde(rename = "205")]
    CarRentalAndTaxiServices,

    #[serde(rename = "1189")]
    CarVideo,

    #[serde(rename = "39")]
    CardGames,

    #[serde(rename = "100")]
    CardsAndGreetings,

    #[serde(rename = "959")]
    CareerResourcesAndPlanning,

    #[serde(rename = "1215")]
    CargoTrucksAndTrailers,

    #[serde(rename = "1246")]
    CarnivalAndMardiGras,

    #[serde(rename = "1339")]
    CarpoolingAndRidesharing,

    #[serde(rename = "319")]
    Cartoons,

    #[serde(rename = "984")]
    CasualApparel,

    #[serde(rename = "926")]
    CasualGames,

    #[serde(rename = "885")]
    Cats,

    #[serde(rename = "184")]
    CelebritiesAndEntertainmentNews,

    #[serde(rename = "57")]
    CharityAndPhilanthropy,

    #[serde(rename = "534")]
    Cheerleading,

    #[serde(rename = "288")]
    ChemicalsIndustry,

    #[serde(rename = "505")]
    Chemistry,

    #[serde(rename = "921")]
    ChessAndAbstractStrategyGames,

    #[serde(rename = "826")]
    Chevrolet,

    #[serde(rename = "403")]
    ChildCare,

    #[serde(rename = "985")]
    ChildrenClothing,

    #[serde(rename = "679")]
    ChildrenInterests,

    #[serde(rename = "1183")]
    ChildrenLiterature,

    #[serde(rename = "741")]
    ChipsAndProcessors,

    #[serde(rename = "643")]
    CholesterolIssues,

    #[serde(rename = "585")]
    ChristianAndGospelMusic,

    #[serde(rename = "1274")]
    ChristianHolidays,

    #[serde(rename = "864")]
    Christianity,

    #[serde(rename = "1078")]
    Christmas,

    #[serde(rename = "833")]
    Chrysler,

    #[serde(rename = "834")]
    Citroën,

    #[serde(rename = "1014")]
    CityAndLocalGuides,

    #[serde(rename = "651")]
    CivilEngineering,

    #[serde(rename = "1102")]
    ClassicFilms,

    #[serde(rename = "1037")]
    ClassicRockAndOldies,

    #[serde(rename = "1013")]
    ClassicVehicles,

    #[serde(rename = "586")]
    ClassicalMusic,

    #[serde(rename = "61")]
    Classifieds,

    #[serde(rename = "671")]
    CleaningAgents,

    #[serde(rename = "949")]
    CleaningSuppliesAndServices,

    #[serde(rename = "1238")]
    CleansingAndDetoxification,

    #[serde(rename = "1255")]
    ClimateChangeAndGlobalWarming,

    #[serde(rename = "1223")]
    ClipArtAndAnimatedGIFs,

    #[serde(rename = "1363")]
    Clocks,

    #[serde(rename = "124")]
    ClothingAccessories,

    #[serde(rename = "188")]
    ClubsAndNightlife,

    #[serde(rename = "189")]
    ClubsAndOrganizations,

    #[serde(rename = "672")]
    CoatingsAndAdhesives,

    #[serde(rename = "916")]
    CoffeeAndTea,

    #[serde(rename = "629")]
    ColdAndFlu,

    #[serde(rename = "923")]
    CollectibleCardGames,

    #[serde(rename = "813")]
    CollegeFinancing,

    #[serde(rename = "1073")]
    CollegeSports,

    #[serde(rename = "372")]
    CollegesAndUniversities,

    #[serde(rename = "514")]
    CombatSports,

    #[serde(rename = "1095")]
    ComedyFilms,

    #[serde(rename = "318")]
    Comics,

    #[serde(rename = "316")]
    ComicsAndAnimation,

    #[serde(rename = "1178")]
    CommercialAndInvestmentRealEstate,

    #[serde(rename = "1160")]
    CommercialLending,

    #[serde(rename = "1214")]
    CommercialVehicles,

    #[serde(rename = "904")]
    CommoditiesAndFuturesTrading,

    #[serde(rename = "1302")]
    CommunicationsAndMediaStudies,

    #[serde(rename = "385")]
    CommunicationsEquipment,

    #[serde(rename = "1240")]
    CompanyEarnings,

    #[serde(rename = "1179")]
    CompanyNews,

    #[serde(rename = "723")]
    CompensationAndBenefits,

    #[serde(rename = "41")]
    ComputerAndVideoGames,

    #[serde(rename = "717")]
    ComputerComponents,

    #[serde(rename = "496")]
    ComputerDrivesAndStorage,

    #[serde(rename = "1229")]
    ComputerEducation,

    #[serde(rename = "30")]
    ComputerHardware,

    #[serde(rename = "226")]
    ComputerMemory,

    #[serde(rename = "487")]
    ComputerMonitorsAndDisplays,

    #[serde(rename = "312")]
    ComputerPeripherals,

    #[serde(rename = "1227")]
    ComputerScience,

    #[serde(rename = "314")]
    ComputerSecurity,

    #[serde(rename = "728")]
    ComputerServers,

    #[serde(rename = "5")]
    ComputersAndElectronics,

    #[serde(rename = "891")]
    ConcertsAndMusicFestivals,

    #[serde(rename = "967")]
    ConstitutionalLawAndCivilRights,

    #[serde(rename = "48")]
    ConstructionAndMaintenance,

    #[serde(rename = "950")]
    ConstructionAndPowerTools,

    #[serde(rename = "652")]
    ConstructionConsultingAndContracting,

    #[serde(rename = "1162")]
    Consulting,

    #[serde(rename = "97")]
    ConsumerAdvocacyAndProtection,

    #[serde(rename = "78")]
    ConsumerElectronics,

    #[serde(rename = "69")]
    ConsumerResources,

    #[serde(rename = "808")]
    ContentManagement,

    #[serde(rename = "1276")]
    ContestsAwardsAndPrizes,

    #[serde(rename = "122")]
    CookingAndRecipes,

    #[serde(rename = "120")]
    CookwareAndDiningware,

    #[serde(rename = "1331")]
    Copiers,

    #[serde(rename = "1181")]
    CorporateAndFinancialCrime,

    #[serde(rename = "334")]
    CorporateEvents,

    #[serde(rename = "331")]
    CorporateTraining,

    #[serde(rename = "1220")]
    CosmeticProcedures,

    #[serde(rename = "238")]
    CosmeticSurgery,

    #[serde(rename = "147")]
    CosmetologyAndBeautyProfessionals,

    #[serde(rename = "988")]
    Costumes,

    #[serde(rename = "511")]
    CounselingServices,

    #[serde(rename = "587")]
    CountryMusic,

    #[serde(rename = "365")]
    CouponsAndDiscountOffers,

    #[serde(rename = "663")]
    CouriersAndMessengers,

    #[serde(rename = "1075")]
    CourtsAndJudiciary,

    #[serde(rename = "284")]
    Crafts,

    #[serde(rename = "279")]
    CreditAndLending,

    #[serde(rename = "811")]
    CreditCards,

    #[serde(rename = "296")]
    Cricket,

    #[serde(rename = "704")]
    CrimeAndJustice,

    #[serde(rename = "424")]
    CriminalLaw,

    #[serde(rename = "749")]
    CropsAndSeed,

    #[serde(rename = "206")]
    CruisesAndCharters,

    #[serde(rename = "297")]
    CulinaryTraining,

    #[serde(rename = "1103")]
    CultAndIndieFilms,

    #[serde(rename = "814")]
    CurrenciesAndForeignExchange,

    #[serde(rename = "806")]
    CustomAndPerformanceVehicles,

    #[serde(rename = "341")]
    CustomerRelationshipManagement,

    #[serde(rename = "450")]
    CustomerServices,

    #[serde(rename = "1373")]
    CutleryAndCuttingAccessories,

    #[serde(rename = "458")]
    Cycling,

    #[serde(rename = "1025")]
    DJResourcesAndEquipment,

    #[serde(rename = "1145")]
    DVDAndVideoRentals,

    #[serde(rename = "210")]
    DVDAndVideoShopping,

    #[serde(rename = "1395")]
    DVDPlayersAndRecorders,

    #[serde(rename = "1393")]
    DVRsAndSetTopBoxes,

    #[serde(rename = "581")]
    Dance,

    #[serde(rename = "588")]
    DanceAndElectronicMusic,

    #[serde(rename = "1323")]
    DataBackupAndRecovery,

    #[serde(rename = "488")]
    DataFormatsAndProtocols,

    #[serde(rename = "343")]
    DataManagement,

    #[serde(rename = "900")]
    DataSheetsAndElectronicsReference,

    #[serde(rename = "55")]
    DatingAndPersonals,

    #[serde(rename = "812")]
    DebtManagement,

    #[serde(rename = "669")]
    DefenseIndustry,

    #[serde(rename = "510")]
    Demographics,

    #[serde(rename = "640")]
    Depression,

    #[serde(rename = "653")]
    Design,

    #[serde(rename = "309")]
    DesktopComputers,

    #[serde(rename = "1088")]
    DesktopPublishing,

    #[serde(rename = "802")]
    DeveloperJobs,

    #[serde(rename = "730")]
    DevelopmentTools,

    #[serde(rename = "225")]
    DeviceDrivers,

    #[serde(rename = "630")]
    Diabetes,

    #[serde(rename = "692")]
    DictionariesAndEncyclopedias,

    #[serde(rename = "917")]
    DiningGuides,

    #[serde(rename = "527")]
    DirectoriesAndListings,

    #[serde(rename = "677")]
    DisabledAndSpecialNeeds,

    #[serde(rename = "1205")]
    DiscriminationAndIdentityRelations,

    #[serde(rename = "367")]
    DistanceLearning,

    #[serde(rename = "1298")]
    DistributedAndParallelComputing,

    #[serde(rename = "664")]
    DistributionAndLogistics,

    #[serde(rename = "1305")]
    DivingAndUnderwaterActivities,

    #[serde(rename = "1261")]
    DivorceAndSeparation,

    #[serde(rename = "634")]
    DoctorsOffices,

    #[serde(rename = "332")]
    DocumentAndPrintingServices,

    #[serde(rename = "1072")]
    DocumentaryFilms,

    #[serde(rename = "836")]
    Dodge,

    #[serde(rename = "886")]
    Dogs,

    #[serde(rename = "472")]
    DomesticServices,

    #[serde(rename = "827")]
    DoorsAndWindows,

    #[serde(rename = "1206")]
    DragAndStreetRacing,

    #[serde(rename = "1094")]
    DramaFilms,

    #[serde(rename = "1397")]
    DrawingAndColoring,

    #[serde(rename = "1173")]
    DressUpAndFashionGames,

    #[serde(rename = "927")]
    DrivingAndRacingGames,

    #[serde(rename = "1351")]
    DrugAndAlcoholTesting,

    #[serde(rename = "1350")]
    DrugAndAlcoholTreatment,

    #[serde(rename = "1314")]
    DrugLawsAndPolicy,

    #[serde(rename = "646")]
    DrugsAndMedications,

    #[serde(rename = "1327")]
    DrumsAndPercussion,

    #[serde(rename = "968")]
    DrunkDrivingLaw,

    #[serde(rename = "673")]
    DyesAndPigments,

    #[serde(rename = "1324")]
    EBookReaders,

    #[serde(rename = "608")]
    EBooks,

    #[serde(rename = "340")]
    ECommerceServices,

    #[serde(rename = "1211")]
    EarNoseAndThroat,

    #[serde(rename = "1012")]
    EarlyChildhoodEducation,

    #[serde(rename = "1168")]
    EarthSciences,

    #[serde(rename = "1033")]
    EastAsianMusic,

    #[serde(rename = "549")]
    EastAsiansAndDiaspora,

    #[serde(rename = "1123")]
    Easter,

    #[serde(rename = "682")]
    EasternEuropeans,

    #[serde(rename = "571")]
    EatingDisorders,

    #[serde(rename = "442")]
    EcologyAndEnvironment,

    #[serde(rename = "520")]
    Economics,

    #[serde(rename = "1164")]
    EconomyNews,

    #[serde(rename = "1005")]
    Ecotourism,

    #[serde(rename = "538")]
    EdgyAndBizarre,

    #[serde(rename = "74")]
    Education,

    #[serde(rename = "374")]
    EducationalResources,

    #[serde(rename = "804")]
    EducationalSoftware,

    #[serde(rename = "1380")]
    ElectricAndPlugInVehicles,

    #[serde(rename = "658")]
    Electricity,

    #[serde(rename = "743")]
    ElectromechanicalDevices,

    #[serde(rename = "1192")]
    ElectronicAccessories,

    #[serde(rename = "742")]
    ElectronicComponents,

    #[serde(rename = "434")]
    ElectronicsAndElectrical,

    #[serde(rename = "394")]
    EmailAndMessaging,

    #[serde(rename = "962")]
    EmbassiesAndConsulates,

    #[serde(rename = "168")]
    EmergencyServices,

    #[serde(rename = "1328")]
    EndocrineConditions,

    #[serde(rename = "233")]
    EnergyAndUtilities,

    #[serde(rename = "1216")]
    EngineAndTransmission,

    #[serde(rename = "231")]
    EngineeringAndTechnology,

    #[serde(rename = "342")]
    EnterpriseResourcePlanning,

    #[serde(rename = "77")]
    EnterpriseTechnology,

    #[serde(rename = "612")]
    EntertainmentIndustry,

    #[serde(rename = "1143")]
    EntertainmentMedia,

    #[serde(rename = "1144")]
    EntertainmentMediaRentals,

    #[serde(rename = "82")]
    EnvironmentalIssues,

    #[serde(rename = "568")]
    Equestrian,

    #[serde(rename = "202")]
    ErectileDysfunction,

    #[serde(rename = "56")]
    EthnicAndIdentityGroups,

    #[serde(rename = "1304")]
    Etiquette,

    #[serde(rename = "956")]
    EventPlanning,

    #[serde(rename = "569")]
    EventsAndListings,

    #[serde(rename = "963")]
    ExecutiveBranch,

    #[serde(rename = "607")]
    ExoticPets,

    #[serde(rename = "973")]
    ExpatriateCommunities,

    #[serde(rename = "1022")]
    ExperimentalAndIndustrialMusic,

    #[serde(rename = "554")]
    ExtremeSports,

    #[serde(rename = "1224")]
    EyeglassesAndContacts,

    #[serde(rename = "989")]
    Eyewear,

    #[serde(rename = "143")]
    FaceAndBodyCare,

    #[serde(rename = "661")]
    FactoryAutomation,

    #[serde(rename = "1132")]
    Family,

    #[serde(rename = "1131")]
    FamilyAndRelationships,

    #[serde(rename = "1291")]
    FamilyFilms,

    #[serde(rename = "522")]
    FamilyLaw,

    #[serde(rename = "1290")]
    FamilyOrientedGamesAndActivities,

    #[serde(rename = "540")]
    FanFiction,

    #[serde(rename = "998")]
    FantasySports,

    #[serde(rename = "185")]
    FashionAndStyle,

    #[serde(rename = "98")]
    FashionDesignersAndCollections,

    #[serde(rename = "1155")]
    FashionModeling,

    #[serde(rename = "918")]
    FastFood,

    #[serde(rename = "1332")]
    FaxMachines,

    #[serde(rename = "1061")]
    Ferrari,

    #[serde(rename = "838")]
    Fiat,

    #[serde(rename = "1230")]
    FiberAndTextileArts,

    #[serde(rename = "928")]
    FightingGames,

    #[serde(rename = "321")]
    FileSharingAndHosting,

    #[serde(rename = "1108")]
    FilmAndTVAwards,

    #[serde(rename = "1116")]
    FilmAndTVIndustry,

    #[serde(rename = "1117")]
    FilmAndTVProduction,

    #[serde(rename = "1086")]
    FilmFestivals,

    #[serde(rename = "7")]
    Finance,

    #[serde(rename = "1163")]
    FinancialMarkets,

    #[serde(rename = "903")]
    FinancialPlanning,

    #[serde(rename = "726")]
    FireAndSecurityServices,

    #[serde(rename = "1165")]
    FiscalPolicyNews,

    #[serde(rename = "887")]
    FishAndAquaria,

    #[serde(rename = "462")]
    Fishing,

    #[serde(rename = "94")]
    Fitness,

    #[serde(rename = "447")]
    FlashBasedEntertainment,

    #[serde(rename = "1318")]
    FlashDrivesAndMemoryCards,

    #[serde(rename = "832")]
    Flooring,

    #[serde(rename = "981")]
    FloraAndFauna,

    #[serde(rename = "323")]
    Flowers,

    #[serde(rename = "1152")]
    FluidHandling,

    #[serde(rename = "1023")]
    FolkAndTraditionalMusic,

    #[serde(rename = "805")]
    Fonts,

    #[serde(rename = "71")]
    FoodAndDrink,

    #[serde(rename = "621")]
    FoodProduction,

    #[serde(rename = "957")]
    FoodService,

    #[serde(rename = "697")]
    Footwear,

    #[serde(rename = "840")]
    Ford,

    #[serde(rename = "1264")]
    ForeignLanguageResources,

    #[serde(rename = "1266")]
    ForeignLanguageStudy,

    #[serde(rename = "750")]
    Forestry,

    #[serde(rename = "990")]
    FormalWear,

    #[serde(rename = "693")]
    FormsGuidesAndTemplates,

    #[serde(rename = "191")]
    ForumAndChatProviders,

    #[serde(rename = "901")]
    FreewareAndShareware,

    #[serde(rename = "289")]
    FreightAndTrucking,

    #[serde(rename = "1134")]
    Friendship,

    #[serde(rename = "908")]
    FruitsAndVegetables,

    #[serde(rename = "1268")]
    FuelEconomyAndGasPrices,

    #[serde(rename = "539")]
    FunAndTrivia,

    #[serde(rename = "1174")]
    FunTestsAndSillySurveys,

    #[serde(rename = "638")]
    GERDAndDigestiveDisorders,

    #[serde(rename = "842")]
    GMC,

    #[serde(rename = "896")]
    GMDaewoo,

    #[serde(rename = "794")]
    GPSAndNavigation,

    #[serde(rename = "362")]
    GadgetsAndPortableElectronics,

    #[serde(rename = "381")]
    GameCheatsAndHints,

    #[serde(rename = "899")]
    GameSystemsAndConsoles,

    #[serde(rename = "8")]
    Games,

    #[serde(rename = "1343")]
    GamingMediaAndReference,

    #[serde(rename = "1312")]
    GangsAndOrganizedCrime,

    #[serde(rename = "269")]
    GardeningAndLandscaping,

    #[serde(rename = "113")]
    GayLesbianBisexualTransgender,

    #[serde(rename = "350")]
    GemsAndJewelry,

    #[serde(rename = "980")]
    GeneralReference,

    #[serde(rename = "835")]
    Generators,

    #[serde(rename = "941")]
    GeneticDisorders,

    #[serde(rename = "982")]
    Genetics,

    #[serde(rename = "1084")]
    GeographicReference,

    #[serde(rename = "443")]
    Geology,

    #[serde(rename = "99")]
    Gifts,

    #[serde(rename = "70")]
    GiftsAndSpecialEventItems,

    #[serde(rename = "261")]
    Golf,

    #[serde(rename = "507")]
    GossipAndTabloidNews,

    #[serde(rename = "503")]
    GothSubculture,

    #[serde(rename = "76")]
    Government,

    #[serde(rename = "1387")]
    GovernmentAgencies,

    #[serde(rename = "1385")]
    GovernmentContractingAndProcurement,

    #[serde(rename = "1282")]
    GrantsAndFinancialAssistance,

    #[serde(rename = "654")]
    GraphicDesign,

    #[serde(rename = "486")]
    GraphicsAndAnimationSoftware,

    #[serde(rename = "121")]
    GroceryAndFoodRetailers,

    #[serde(rename = "1325")]
    Guitars,

    #[serde(rename = "519")]
    Gymnastics,

    #[serde(rename = "1354")]
    HDTVs,

    #[serde(rename = "828")]
    HVACAndClimateControl,

    #[serde(rename = "146")]
    HairCare,

    #[serde(rename = "235")]
    HairLoss,

    #[serde(rename = "1079")]
    HalloweenAndOctober31st,

    #[serde(rename = "986")]
    HandbagsAndPurses,

    #[serde(rename = "1017")]
    Handball,

    #[serde(rename = "1046")]
    HandheldGameConsoles,

    #[serde(rename = "1320")]
    HardDrives,

    #[serde(rename = "1035")]
    HardRockAndProgressive,

    #[serde(rename = "739")]
    HardwareModdingAndTuning,

    #[serde(rename = "631")]
    HeadachesAndMigraines,

    #[serde(rename = "1396")]
    Headphones,

    #[serde(rename = "991")]
    Headwear,

    #[serde(rename = "45")]
    Health,

    #[serde(rename = "419")]
    HealthConditions,

    #[serde(rename = "254")]
    HealthEducationAndMedicalTraining,

    #[serde(rename = "252")]
    HealthFoundationsAndMedicalResearch,

    #[serde(rename = "249")]
    HealthInsurance,

    #[serde(rename = "1253")]
    HealthNews,

    #[serde(rename = "1256")]
    HealthPolicy,

    #[serde(rename = "559")]
    HeartAndHypertension,

    #[serde(rename = "837")]
    HeavyMachinery,

    #[serde(rename = "542")]
    HikingAndCamping,

    #[serde(rename = "866")]
    Hinduism,

    #[serde(rename = "1006")]
    HistoricalSitesAndBuildings,

    #[serde(rename = "433")]
    History,

    #[serde(rename = "65")]
    HobbiesAndLeisure,

    #[serde(rename = "260")]
    Hockey,

    #[serde(rename = "678")]
    HolidaysAndSeasonalEvents,

    #[serde(rename = "11")]
    HomeAndGarden,

    #[serde(rename = "271")]
    HomeAppliances,

    #[serde(rename = "466")]
    HomeFinancing,

    #[serde(rename = "270")]
    HomeFurnishings,

    #[serde(rename = "158")]
    HomeImprovement,

    #[serde(rename = "465")]
    HomeInsurance,

    #[serde(rename = "727")]
    HomeOffice,

    #[serde(rename = "1348")]
    HomeStorageAndShelving,

    #[serde(rename = "1157")]
    HomeTheaterSystems,

    #[serde(rename = "137")]
    HomemakingAndInteriorDecor,

    #[serde(rename = "791")]
    Homeschooling,

    #[serde(rename = "843")]
    Honda,

    #[serde(rename = "615")]
    HorrorFilms,

    #[serde(rename = "888")]
    Horses,

    #[serde(rename = "751")]
    Horticulture,

    #[serde(rename = "955")]
    HospitalityIndustry,

    #[serde(rename = "250")]
    HospitalsAndTreatmentCenters,

    #[serde(rename = "179")]
    HotelsAndAccommodations,

    #[serde(rename = "1232")]
    HousePaintingAndFinishing,

    #[serde(rename = "1166")]
    HousingAndDevelopment,

    #[serde(rename = "694")]
    HowToDIYAndExpertContent,

    #[serde(rename = "157")]
    HumanResources,

    #[serde(rename = "1280")]
    HumanRightsAndLiberties,

    #[serde(rename = "474")]
    Humanities,

    #[serde(rename = "1062")]
    Hummer,

    #[serde(rename = "182")]
    Humor,

    #[serde(rename = "461")]
    HuntingAndShooting,

    #[serde(rename = "810")]
    HybridAndAlternativeVehicles,

    #[serde(rename = "244")]
    HygieneAndToiletries,

    #[serde(rename = "845")]
    Hyundai,

    #[serde(rename = "104")]
    ISPs,

    #[serde(rename = "1149")]
    IceSkating,

    #[serde(rename = "1313")]
    ImmigrationPolicyAndBorderIssues,

    #[serde(rename = "354")]
    ImportAndExport,

    #[serde(rename = "1038")]
    IndieAndAlternativeMusic,

    #[serde(rename = "681")]
    IndigenousPeoples,

    #[serde(rename = "1000")]
    IndividualSports,

    #[serde(rename = "655")]
    IndustrialAndProductDesign,

    #[serde(rename = "287")]
    IndustrialMaterialsAndEquipment,

    #[serde(rename = "632")]
    InfectiousDiseases,

    #[serde(rename = "647")]
    Infertility,

    #[serde(rename = "1377")]
    Infiniti,

    #[serde(rename = "817")]
    Injury,

    #[serde(rename = "1333")]
    InkAndToner,

    #[serde(rename = "493")]
    InputDevices,

    #[serde(rename = "1278")]
    InsectsAndEntomology,

    #[serde(rename = "38")]
    Insurance,

    #[serde(rename = "426")]
    IntellectualProperty,

    #[serde(rename = "1221")]
    IntelligenceAndCounterterrorism,

    #[serde(rename = "656")]
    InteriorDesign,

    #[serde(rename = "521")]
    InternationalRelations,

    #[serde(rename = "13")]
    InternetAndTelecom,

    #[serde(rename = "304")]
    InternetClientsAndBrowsers,

    #[serde(rename = "807")]
    InternetSoftware,

    #[serde(rename = "107")]
    Investing,

    #[serde(rename = "1139")]
    InvestmentBanking,

    #[serde(rename = "868")]
    Islam,

    #[serde(rename = "1275")]
    IslamicHolidays,

    #[serde(rename = "1378")]
    Isuzu,

    #[serde(rename = "1063")]
    Jaguar,

    #[serde(rename = "732")]
    Java,

    #[serde(rename = "42")]
    Jazz,

    #[serde(rename = "589")]
    JazzAndBlues,

    #[serde(rename = "846")]
    Jeep,

    #[serde(rename = "550")]
    JewishCulture,

    #[serde(rename = "1124")]
    JewishHolidays,

    #[serde(rename = "960")]
    JobListings,

    #[serde(rename = "60")]
    Jobs,

    #[serde(rename = "958")]
    JobsAndEducation,

    #[serde(rename = "1204")]
    JournalismAndNewsIndustry,

    #[serde(rename = "869")]
    Judaism,

    #[serde(rename = "848")]
    Kia,

    #[serde(rename = "154")]
    KidsAndTeens,

    #[serde(rename = "951")]
    KitchenAndDining,

    #[serde(rename = "800")]
    KnowledgeManagement,

    #[serde(rename = "1356")]
    LCDTVs,

    #[serde(rename = "701")]
    LaborAndEmploymentLaw,

    #[serde(rename = "1120")]
    LakesAndRivers,

    #[serde(rename = "1064")]
    Lamborghini,

    #[serde(rename = "272")]
    LampsAndLighting,

    #[serde(rename = "1065")]
    LandRover,

    #[serde(rename = "108")]
    LanguageResources,

    #[serde(rename = "310")]
    LaptopsAndNotebooks,

    #[serde(rename = "913")]
    LatinAmericanCuisine,

    #[serde(rename = "591")]
    LatinAmericanMusic,

    #[serde(rename = "1285")]
    LatinPop,

    #[serde(rename = "548")]
    LatinosAndLatinAmericans,

    #[serde(rename = "1364")]
    Laundry,

    #[serde(rename = "19")]
    LawAndGovernment,

    #[serde(rename = "535")]
    LawEnforcement,

    #[serde(rename = "641")]
    LearningAndDevelopmentalDisabilities,

    #[serde(rename = "410")]
    LeftWingPolitics,

    #[serde(rename = "75")]
    Legal,

    #[serde(rename = "792")]
    LegalEducation,

    #[serde(rename = "1137")]
    LegalForms,

    #[serde(rename = "969")]
    LegalServices,

    #[serde(rename = "964")]
    LegislativeBranch,

    #[serde(rename = "849")]
    Lexus,

    #[serde(rename = "375")]
    LibrariesAndMuseums,

    #[serde(rename = "850")]
    Lincoln,

    #[serde(rename = "736")]
    LinuxAndUnix,

    #[serde(rename = "406")]
    Liquor,

    #[serde(rename = "1184")]
    LiteraryClassics,

    #[serde(rename = "895")]
    LiveComedy,

    #[serde(rename = "1273")]
    LiveSportingEvents,

    #[serde(rename = "752")]
    Livestock,

    #[serde(rename = "1386")]
    Lobbying,

    #[serde(rename = "572")]
    LocalNews,

    #[serde(rename = "364")]
    LotteryAndSweepstakes,

    #[serde(rename = "1309")]
    LoyaltyCardsAndPrograms,

    #[serde(rename = "1003")]
    LuggageAndTravelAccessories,

    #[serde(rename = "696")]
    LuxuryGoods,

    #[serde(rename = "552")]
    MLMAndBusinessOpportunities,

    #[serde(rename = "227")]
    MP3AndPortableMediaPlayers,

    #[serde(rename = "735")]
    MacOS,

    #[serde(rename = "1299")]
    MachineLearningAndArtificialIntelligence,

    #[serde(rename = "412")]
    Magazines,

    #[serde(rename = "1150")]
    MailAndPackageDelivery,

    #[serde(rename = "1293")]
    MajorKitchenAppliances,

    #[serde(rename = "234")]
    MakeUpAndCosmetics,

    #[serde(rename = "338")]
    Management,

    #[serde(rename = "49")]
    Manufacturing,

    #[serde(rename = "268")]
    Maps,

    #[serde(rename = "1250")]
    Marines,

    #[serde(rename = "665")]
    MaritimeTransport,

    #[serde(rename = "83")]
    MarketingServices,

    #[serde(rename = "1133")]
    Marriage,

    #[serde(rename = "516")]
    MartialArts,

    #[serde(rename = "1101")]
    MartialArtsFilms,

    #[serde(rename = "1066")]
    Maserati,

    #[serde(rename = "73")]
    MassMerchantsAndDepartmentStores,

    #[serde(rename = "557")]
    MassageTherapy,

    #[serde(rename = "935")]
    MassiveMultiplayer,

    #[serde(rename = "436")]
    Mathematics,

    #[serde(rename = "546")]
    MatrimonialServices,

    #[serde(rename = "1368")]
    Mattresses,

    #[serde(rename = "851")]
    Mazda,

    #[serde(rename = "909")]
    MeatAndSeafood,

    #[serde(rename = "1203")]
    MediaCriticsAndWatchdogs,

    #[serde(rename = "1090")]
    MediaPlayers,

    #[serde(rename = "251")]
    MedicalDevicesAndEquipment,

    #[serde(rename = "256")]
    MedicalFacilitiesAndServices,

    #[serde(rename = "253")]
    MedicalLiteratureAndResources,

    #[serde(rename = "945")]
    MedicalPhotosAndIllustration,

    #[serde(rename = "635")]
    MedicalProcedures,

    #[serde(rename = "943")]
    MedicalTestsAndExams,

    #[serde(rename = "914")]
    MediterraneanCuisine,

    #[serde(rename = "1319")]
    MemoryCardReaders,

    #[serde(rename = "992")]
    MenClothing,

    #[serde(rename = "636")]
    MenHealth,

    #[serde(rename = "437")]
    MentalHealth,

    #[serde(rename = "852")]
    MercedesBenz,

    #[serde(rename = "280")]
    MerchantServicesAndPaymentSystems,

    #[serde(rename = "853")]
    Mercury,

    #[serde(rename = "1241")]
    MergersAndAcquisitions,

    #[serde(rename = "1036")]
    MetalMusic,

    #[serde(rename = "606")]
    MetalsAndMining,

    #[serde(rename = "1381")]
    Microblogging,

    #[serde(rename = "1317")]
    MicrocarsAndCityCars,

    #[serde(rename = "366")]
    Military,

    #[serde(rename = "1288")]
    MilitaryHistory,

    #[serde(rename = "1067")]
    Mini,

    #[serde(rename = "922")]
    MiniaturesAndWargaming,

    #[serde(rename = "854")]
    Mitsubishi,

    #[serde(rename = "382")]
    MobileAndWireless,

    #[serde(rename = "1171")]
    MobileAndWirelessAccessories,

    #[serde(rename = "1109")]
    MobileAppsAndAddOns,

    #[serde(rename = "1382")]
    MobileOS,

    #[serde(rename = "390")]
    MobilePhones,

    #[serde(rename = "1353")]
    MobilityEquipmentAndAccessories,

    #[serde(rename = "180")]
    MotorSports,

    #[serde(rename = "273")]
    Motorcycles,

    #[serde(rename = "1119")]
    MountainAndSkiResorts,

    #[serde(rename = "1085")]
    MovieListingsAndTheaterShowtimes,

    #[serde(rename = "213")]
    MovieMemorabilia,

    #[serde(rename = "1106")]
    MovieReference,

    #[serde(rename = "1107")]
    MovieReviewsAndPreviews,

    #[serde(rename = "34")]
    Movies,

    #[serde(rename = "291")]
    MovingAndRelocation,

    #[serde(rename = "965")]
    MultilateralOrganizations,

    #[serde(rename = "497")]
    MultimediaSoftware,

    #[serde(rename = "35")]
    MusicAndAudio,

    #[serde(rename = "929")]
    MusicAndDanceGames,

    #[serde(rename = "218")]
    MusicArtAndMemorabilia,

    #[serde(rename = "1113")]
    MusicAwards,

    #[serde(rename = "1028")]
    MusicCompositionAndTheory,

    #[serde(rename = "1087")]
    MusicEducationAndInstruction,

    #[serde(rename = "1024")]
    MusicEquipmentAndTechnology,

    #[serde(rename = "1026")]
    MusicRecordingTechnology,

    #[serde(rename = "1027")]
    MusicReference,

    #[serde(rename = "220")]
    MusicStreamsAndDownloads,

    #[serde(rename = "1105")]
    MusicalFilms,

    #[serde(rename = "216")]
    MusicalInstruments,

    #[serde(rename = "609")]
    MythAndFolklore,

    #[serde(rename = "829")]
    NailsScrewsAndFasteners,

    #[serde(rename = "171")]
    NativeAmericans,

    #[serde(rename = "1249")]
    Navy,

    #[serde(rename = "347")]
    NetworkMonitoringAndManagement,

    #[serde(rename = "344")]
    NetworkSecurity,

    #[serde(rename = "729")]
    NetworkStorage,

    #[serde(rename = "311")]
    Networking,

    #[serde(rename = "346")]
    NetworkingEquipment,

    #[serde(rename = "942")]
    NeurologicalDisorders,

    #[serde(rename = "1226")]
    Neuroscience,

    #[serde(rename = "1271")]
    NewYear,

    #[serde(rename = "16")]
    News,

    #[serde(rename = "408")]
    Newspapers,

    #[serde(rename = "1043")]
    Nintendo,

    #[serde(rename = "855")]
    Nissan,

    #[serde(rename = "560")]
    NonAlcoholicBeverages,

    #[serde(rename = "915")]
    NorthAmericanCuisine,

    #[serde(rename = "954")]
    NuclearEnergy,

    #[serde(rename = "1372")]
    NurseryAndPlayroom,

    #[serde(rename = "418")]
    Nursing,

    #[serde(rename = "456")]
    Nutrition,

    #[serde(rename = "558")]
    OBGYN,

    #[serde(rename = "818")]
    Obesity,

    #[serde(rename = "449")]
    OccultAndParanormal,

    #[serde(rename = "644")]
    OccupationalHealthAndSafety,

    #[serde(rename = "148")]
    OffRoadVehicles,

    #[serde(rename = "33")]
    Offbeat,

    #[serde(rename = "337")]
    OfficeAndFacilitiesManagement,

    #[serde(rename = "333")]
    OfficeFurniture,

    #[serde(rename = "28")]
    OfficeServices,

    #[serde(rename = "95")]
    OfficeSupplies,

    #[serde(rename = "659")]
    OilAndGas,

    #[serde(rename = "513")]
    Olympics,

    #[serde(rename = "299")]
    OnlineCommunities,

    #[serde(rename = "105")]
    OnlineGames,

    #[serde(rename = "43")]
    OnlineGoodies,

    #[serde(rename = "1222")]
    OnlineImageGalleries,

    #[serde(rename = "582")]
    OnlineJournalsAndPersonalSites,

    #[serde(rename = "613")]
    OnlineMedia,

    #[serde(rename = "211")]
    OnlineVideo,

    #[serde(rename = "313")]
    OpenSource,

    #[serde(rename = "1185")]
    Opera,

    #[serde(rename = "303")]
    OperatingSystems,

    #[serde(rename = "1201")]
    OpinionAndCommentary,

    #[serde(rename = "744")]
    OptoelectronicsAndFiber,

    #[serde(rename = "245")]
    OralAndDentalCare,

    #[serde(rename = "688")]
    Outdoors,

    #[serde(rename = "993")]
    Outerwear,

    #[serde(rename = "718")]
    Outsourcing,

    #[serde(rename = "228")]
    PDAsAndHandhelds,

    #[serde(rename = "290")]
    Packaging,

    #[serde(rename = "1258")]
    PaganAndEsotericTraditions,

    #[serde(rename = "819")]
    PainManagement,

    #[serde(rename = "786")]
    Paintball,

    #[serde(rename = "1167")]
    Painting,

    #[serde(rename = "1169")]
    Paleontology,

    #[serde(rename = "1262")]
    ParasitesAndParasiticDiseases,

    #[serde(rename = "58")]
    Parenting,

    #[serde(rename = "1306")]
    Parking,

    #[serde(rename = "324")]
    PartyAndHolidaySupplies,

    #[serde(rename = "936")]
    PartyGames,

    #[serde(rename = "724")]
    PayrollServices,

    #[serde(rename = "645")]
    Pediatrics,

    #[serde(rename = "14")]
    PeopleAndSociety,

    #[serde(rename = "1234")]
    PeopleSearch,

    #[serde(rename = "23")]
    PerformingArts,

    #[serde(rename = "242")]
    PerfumesAndFragrances,

    #[serde(rename = "1147")]
    PersonalAircraft,

    #[serde(rename = "102")]
    Personals,

    #[serde(rename = "471")]
    PestControl,

    #[serde(rename = "379")]
    PetFoodAndSupplies,

    #[serde(rename = "563")]
    Pets,

    #[serde(rename = "66")]
    PetsAndAnimals,

    #[serde(rename = "856")]
    Peugeot,

    #[serde(rename = "255")]
    PharmaceuticalsAndBiotech,

    #[serde(rename = "248")]
    Pharmacy,

    #[serde(rename = "1093")]
    Philosophy,

    #[serde(rename = "384")]
    PhoneServiceProviders,

    #[serde(rename = "978")]
    PhotoAndImageSharing,

    #[serde(rename = "576")]
    PhotoAndVideoServices,

    #[serde(rename = "275")]
    PhotoAndVideoSharing,

    #[serde(rename = "577")]
    PhotoAndVideoSoftware,

    #[serde(rename = "320")]
    PhotoRatingSites,

    #[serde(rename = "439")]
    PhotographicAndDigitalArts,

    #[serde(rename = "719")]
    PhysicalAssetManagement,

    #[serde(rename = "500")]
    PhysicalTherapy,

    #[serde(rename = "444")]
    Physics,

    #[serde(rename = "1326")]
    PianosAndKeyboards,

    #[serde(rename = "1296")]
    PlacesofWorship,

    #[serde(rename = "1355")]
    PlasmaTVs,

    #[serde(rename = "674")]
    PlasticsAndPolymers,

    #[serde(rename = "1153")]
    Plumbing,

    #[serde(rename = "830")]
    PlumbingFixturesAndEquipment,

    #[serde(rename = "809")]
    Podcasting,

    #[serde(rename = "565")]
    Poetry,

    #[serde(rename = "946")]
    PoisonsAndOverdoses,

    #[serde(rename = "924")]
    PokerAndCasinoGames,

    #[serde(rename = "1180")]
    PoliticalHumor,

    #[serde(rename = "1202")]
    PoliticalPollsAndSurveys,

    #[serde(rename = "396")]
    Politics,

    #[serde(rename = "857")]
    Pontiac,

    #[serde(rename = "1021")]
    PopMusic,

    #[serde(rename = "858")]
    Porsche,

    #[serde(rename = "1127")]
    PovertyAndHunger,

    #[serde(rename = "745")]
    PowerSupplies,

    #[serde(rename = "401")]
    PregnancyAndMaternity,

    #[serde(rename = "1346")]
    PresentationSoftware,

    #[serde(rename = "352")]
    PriceComparisons,

    #[serde(rename = "371")]
    PrimaryAndSecondarySchooling,

    #[serde(rename = "494")]
    Printers,

    #[serde(rename = "1330")]
    PrintersCopiersAndFax,

    #[serde(rename = "1176")]
    PrintingAndPublishing,

    #[serde(rename = "1284")]
    PrisonsAndCorrections,

    #[serde(rename = "1281")]
    PrivacyIssues,

    #[serde(rename = "970")]
    ProductLiability,

    #[serde(rename = "353")]
    ProductReviewsAndPriceComparisons,

    #[serde(rename = "1199")]
    ProfessionalAndTradeAssociations,

    #[serde(rename = "31")]
    Programming,

    #[serde(rename = "1360")]
    ProjectManagement,

    #[serde(rename = "1359")]
    ProjectManagementSoftware,

    #[serde(rename = "1357")]
    ProjectionTVs,

    #[serde(rename = "1334")]
    ProjectorsAndScreens,

    #[serde(rename = "687")]
    PropertyDevelopment,

    #[serde(rename = "463")]
    PropertyInspectionsAndAppraisals,

    #[serde(rename = "425")]
    PropertyManagement,

    #[serde(rename = "902")]
    ProxyingAndFiltering,

    #[serde(rename = "543")]
    Psychology,

    #[serde(rename = "1161")]
    PublicFinance,

    #[serde(rename = "947")]
    PublicHealth,

    #[serde(rename = "1316")]
    PublicPolicy,

    #[serde(rename = "1136")]
    PublicRecords,

    #[serde(rename = "327")]
    PublicRelations,

    #[serde(rename = "166")]
    PublicSafety,

    #[serde(rename = "1303")]
    PublicSpeaking,

    #[serde(rename = "1347")]
    PublicStorage,

    #[serde(rename = "1041")]
    PunkMusic,

    #[serde(rename = "937")]
    PuzzlesAndBrainteasers,

    #[serde(rename = "720")]
    QualityControlAndTracking,

    #[serde(rename = "889")]
    RabbitsAndRodents,

    #[serde(rename = "262")]
    RacquetSports,

    #[serde(rename = "215")]
    Radio,

    #[serde(rename = "787")]
    RadioControlAndModeling,

    #[serde(rename = "1182")]
    RadioEquipment,

    #[serde(rename = "666")]
    RailTransport,

    #[serde(rename = "1030")]
    RapAndHipHop,

    #[serde(rename = "29")]
    RealEstate,

    #[serde(rename = "96")]
    RealEstateAgencies,

    #[serde(rename = "1080")]
    RealEstateListings,

    #[serde(rename = "1114")]
    RecordLabels,

    #[serde(rename = "1115")]
    RecordingIndustry,

    #[serde(rename = "999")]
    RecreationalAviation,

    #[serde(rename = "330")]
    RecruitmentAndStaffing,

    #[serde(rename = "1307")]
    Recycling,

    #[serde(rename = "533")]
    Reference,

    #[serde(rename = "1031")]
    ReggaeAndCaribbeanMusic,

    #[serde(rename = "1242")]
    Reggaeton,

    #[serde(rename = "1007")]
    RegionalParksAndGardens,

    #[serde(rename = "59")]
    ReligionAndBelief,

    #[serde(rename = "1020")]
    ReligiousMusic,

    #[serde(rename = "859")]
    RenaultSamsung,

    #[serde(rename = "657")]
    RenewableAndAlternativeEnergy,

    #[serde(rename = "195")]
    ReproductiveHealth,

    #[serde(rename = "976")]
    ReproductiveRights,

    #[serde(rename = "890")]
    ReptilesAndAmphibians,

    #[serde(rename = "824")]
    RespiratoryConditions,

    #[serde(rename = "816")]
    RestaurantSupply,

    #[serde(rename = "276")]
    Restaurants,

    #[serde(rename = "961")]
    ResumesAndPortfolios,

    #[serde(rename = "844")]
    RetailEquipmentAndTechnology,

    #[serde(rename = "841")]
    RetailTrade,

    #[serde(rename = "619")]
    RetirementAndPension,

    #[serde(rename = "409")]
    RightWingPolitics,

    #[serde(rename = "532")]
    RingtonesAndMobileGoodies,

    #[serde(rename = "620")]
    RiskManagement,

    #[serde(rename = "1141")]
    Robotics,

    #[serde(rename = "590")]
    RockMusic,

    #[serde(rename = "622")]
    RoleplayingGames,

    #[serde(rename = "1068")]
    RollsRoyce,

    #[serde(rename = "1135")]
    Romance,

    #[serde(rename = "1310")]
    RomanceFilms,

    #[serde(rename = "1175")]
    Roofing,

    #[serde(rename = "702")]
    Royalty,

    #[serde(rename = "517")]
    Rugby,

    #[serde(rename = "1362")]
    RugsAndCarpets,

    #[serde(rename = "541")]
    RunningAndWalking,

    #[serde(rename = "1057")]
    SUVs,

    #[serde(rename = "897")]
    Saab,

    #[serde(rename = "1286")]
    SalsaAndTropicalMusic,

    #[serde(rename = "1301")]
    SameSexMarriage,

    #[serde(rename = "1091")]
    SamplesAndSoundLibraries,

    #[serde(rename = "860")]
    Saturn,

    #[serde(rename = "1259")]
    ScandalsAndInvestigations,

    #[serde(rename = "495")]
    Scanners,

    #[serde(rename = "174")]
    Science,

    #[serde(rename = "676")]
    ScienceFictionAndFantasy,

    #[serde(rename = "616")]
    ScienceFictionAndFantasyFilms,

    #[serde(rename = "445")]
    ScientificEquipment,

    #[serde(rename = "446")]
    ScientificInstitutions,

    #[serde(rename = "1251")]
    Scientology,

    #[serde(rename = "1069")]
    Scion,

    #[serde(rename = "1212")]
    ScootersAndMopeds,

    #[serde(rename = "733")]
    ScriptingLanguages,

    #[serde(rename = "84")]
    SearchEngineOptimizationAndMarketing,

    #[serde(rename = "485")]
    SearchEngines,

    #[serde(rename = "705")]
    SecurityProductsAndServices,

    #[serde(rename = "870")]
    SelfHelpAndMotivational,

    #[serde(rename = "298")]
    SeniorsAndRetirement,

    #[serde(rename = "383")]
    ServiceProviders,

    #[serde(rename = "536")]
    SexEducationAndCounseling,

    #[serde(rename = "1236")]
    SexualEnhancement,

    #[serde(rename = "421")]
    SexuallyTransmittedDiseases,

    #[serde(rename = "892")]
    SheetMusic,

    #[serde(rename = "930")]
    ShooterGames,

    #[serde(rename = "18")]
    Shopping,

    #[serde(rename = "531")]
    ShoppingPortalsAndSearchEngines,

    #[serde(rename = "1390")]
    SightseeingTours,

    #[serde(rename = "1076")]
    Signage,

    #[serde(rename = "1098")]
    SilentFilms,

    #[serde(rename = "931")]
    SimulationGames,

    #[serde(rename = "1126")]
    SkateSports,

    #[serde(rename = "975")]
    SkepticsAndNonBelievers,

    #[serde(rename = "1148")]
    SkiingAndSnowboarding,

    #[serde(rename = "93")]
    SkinAndNailCare,

    #[serde(rename = "420")]
    SkinConditions,

    #[serde(rename = "578")]
    SkinsThemesAndWallpapers,

    #[serde(rename = "633")]
    SleepDisorders,

    #[serde(rename = "994")]
    Sleepwear,

    #[serde(rename = "551")]
    SmallBusiness,

    #[serde(rename = "1292")]
    SmallKitchenAppliances,

    #[serde(rename = "1071")]
    SmartPhones,

    #[serde(rename = "1237")]
    SmokingAndSmokingCessation,

    #[serde(rename = "294")]
    Soccer,

    #[serde(rename = "54")]
    SocialIssuesAndAdvocacy,

    #[serde(rename = "847")]
    SocialNetworkAppsAndAddOns,

    #[serde(rename = "529")]
    SocialNetworks,

    #[serde(rename = "509")]
    SocialSciences,

    #[serde(rename = "508")]
    SocialServices,

    #[serde(rename = "1370")]
    SofasAndChairs,

    #[serde(rename = "32")]
    Software,

    #[serde(rename = "224")]
    SoftwareUtilities,

    #[serde(rename = "617")]
    SongLyricsAndTabs,

    #[serde(rename = "1044")]
    SonyPlayStation,

    #[serde(rename = "1039")]
    SoulAndRAndB,

    #[serde(rename = "740")]
    SoundAndVideoCards,

    #[serde(rename = "893")]
    Soundtracks,

    #[serde(rename = "910")]
    SoupsAndStews,

    #[serde(rename = "1032")]
    SouthAsianMusic,

    #[serde(rename = "528")]
    SouthAsiansAndDiaspora,

    #[serde(rename = "580")]
    SoutheastAsiansAndPacificIslanders,

    #[serde(rename = "668")]
    SpaceTechnology,

    #[serde(rename = "145")]
    SpasAndBeautyServices,

    #[serde(rename = "1158")]
    Speakers,

    #[serde(rename = "457")]
    SpecialAndRestrictedDiets,

    #[serde(rename = "1118")]
    SpecialEducation,

    #[serde(rename = "977")]
    SpecialOccasions,

    #[serde(rename = "1004")]
    SpecialtyTravel,

    #[serde(rename = "101")]
    Spirituality,

    #[serde(rename = "1244")]
    SpoofsAndSatire,

    #[serde(rename = "263")]
    SportingGoods,

    #[serde(rename = "20")]
    Sports,

    #[serde(rename = "1082")]
    SportsCoachingAndTraining,

    #[serde(rename = "932")]
    SportsGames,

    #[serde(rename = "1083")]
    SportsMemorabilia,

    #[serde(rename = "1077")]
    SportsNews,

    #[serde(rename = "1344")]
    SpreadsheetSoftware,

    #[serde(rename = "373")]
    StandardizedAndAdmissionsTests,

    #[serde(rename = "966")]
    StateAndLocalGovernment,

    #[serde(rename = "1252")]
    Statistics,

    #[serde(rename = "91")]
    StereoSystemsAndComponents,

    #[serde(rename = "1235")]
    SteroidsAndPerformanceEnhancingDrugs,

    #[serde(rename = "574")]
    StockPhotography,

    #[serde(rename = "722")]
    StrategicPlanning,

    #[serde(rename = "933")]
    StrategyGames,

    #[serde(rename = "1308")]
    StudyAbroad,

    #[serde(rename = "1207")]
    StuntsAndDangerousFeats,

    #[serde(rename = "861")]
    Subaru,

    #[serde(rename = "502")]
    SubculturesAndNicheInterests,

    #[serde(rename = "257")]
    SubstanceAbuse,

    #[serde(rename = "1100")]
    SuperheroFilms,

    #[serde(rename = "801")]
    SupplyChainManagement,

    #[serde(rename = "689")]
    SurfAndSwim,

    #[serde(rename = "944")]
    Surgery,

    #[serde(rename = "1070")]
    Suzuki,

    #[serde(rename = "1210")]
    SwapMeetsAndOutdoorMarkets,

    #[serde(rename = "952")]
    SwimmingPoolsAndSpas,

    #[serde(rename = "995")]
    Swimwear,

    #[serde(rename = "428")]
    TShirts,

    #[serde(rename = "36")]
    TVAndVideo,

    #[serde(rename = "229")]
    TVAndVideoEquipment,

    #[serde(rename = "1047")]
    TVComedies,

    #[serde(rename = "1055")]
    TVCommercials,

    #[serde(rename = "1111")]
    TVCrimeAndLegalShows,

    #[serde(rename = "1193")]
    TVDramas,

    #[serde(rename = "1110")]
    TVFamilyOrientedShows,

    #[serde(rename = "1050")]
    TVGameShows,

    #[serde(rename = "1187")]
    TVGuidesAndReference,

    #[serde(rename = "1194")]
    TVMedicalShows,

    #[serde(rename = "359")]
    TVNetworksAndStations,

    #[serde(rename = "1049")]
    TVRealityShows,

    #[serde(rename = "1112")]
    TVSciFiAndFantasyShows,

    #[serde(rename = "358")]
    TVShowsAndPrograms,

    #[serde(rename = "357")]
    TVSoapOperas,

    #[serde(rename = "1048")]
    TVTalkShows,

    #[serde(rename = "938")]
    TableGames,

    #[serde(rename = "940")]
    TableTennis,

    #[serde(rename = "1277")]
    TabletPCs,

    #[serde(rename = "1186")]
    TalkRadio,

    #[serde(rename = "1283")]
    TaxPreparationAndPlanning,

    #[serde(rename = "700")]
    TeachingAndClassroomResources,

    #[serde(rename = "1001")]
    TeamSports,

    #[serde(rename = "1233")]
    TechnicalReference,

    #[serde(rename = "567")]
    TechnicalSupport,

    #[serde(rename = "785")]
    TechnologyNews,

    #[serde(rename = "680")]
    TeenInterests,

    #[serde(rename = "392")]
    Teleconferencing,

    #[serde(rename = "328")]
    Telemarketing,

    #[serde(rename = "305")]
    Televisions,

    #[serde(rename = "1376")]
    Tennis,

    #[serde(rename = "746")]
    TestAndMeasurement,

    #[serde(rename = "1379")]
    TextAndInstantMessaging,

    #[serde(rename = "566")]
    TextilesAndNonwovens,

    #[serde(rename = "1125")]
    Thanksgiving,

    #[serde(rename = "1008")]
    ThemeParks,

    #[serde(rename = "1340")]
    TheologyAndReligiousStudy,

    #[serde(rename = "1096")]
    ThrillerCrimeAndMysteryFilms,

    #[serde(rename = "1329")]
    ThyroidConditions,

    #[serde(rename = "614")]
    TicketSales,

    #[serde(rename = "695")]
    TimeAndCalendars,

    #[serde(rename = "1081")]
    TimesharesAndVacationProperties,

    #[serde(rename = "123")]
    TobaccoProducts,

    #[serde(rename = "1392")]
    TouristBoardsAndVisitorCenters,

    #[serde(rename = "208")]
    TouristDestinations,

    #[serde(rename = "863")]
    Toyota,

    #[serde(rename = "432")]
    Toys,

    #[serde(rename = "518")]
    TrackAndField,

    #[serde(rename = "335")]
    TradeShowsAndConventions,

    #[serde(rename = "685")]
    TrafficAndPublicTransit,

    #[serde(rename = "1388")]
    TrainingAndCertification,

    #[serde(rename = "1265")]
    TranslationToolsAndResources,

    #[serde(rename = "50")]
    TransportationAndLogistics,

    #[serde(rename = "6")]
    Travel,

    #[serde(rename = "1010")]
    TravelAgenciesAndServices,

    #[serde(rename = "1011")]
    TravelGuidesAndTravelogues,

    #[serde(rename = "1260")]
    TroubledRelationships,

    #[serde(rename = "1056")]
    Trucks,

    #[serde(rename = "610")]
    TrucksAndSUVs,

    #[serde(rename = "530")]
    Undergarments,

    #[serde(rename = "996")]
    UniformsAndWorkwear,

    #[serde(rename = "1121")]
    UnionsAndLaborMovement,

    #[serde(rename = "144")]
    UnwantedBodyAndFacialHairRemoval,

    #[serde(rename = "592")]
    UrbanAndHipHop,

    #[serde(rename = "686")]
    UrbanAndRegionalPlanning,

    #[serde(rename = "667")]
    UrbanTransport,

    #[serde(rename = "1279")]
    VPNAndRemoteAccess,

    #[serde(rename = "1019")]
    VacationOffers,

    #[serde(rename = "1263")]
    VaccinesAndImmunizations,

    #[serde(rename = "1122")]
    ValentineDay,

    #[serde(rename = "839")]
    ValvesHosesAndFittings,

    #[serde(rename = "1058")]
    VansAndMinivans,

    #[serde(rename = "898")]
    VauxhallOpel,

    #[serde(rename = "825")]
    VegetarianCuisine,

    #[serde(rename = "815")]
    VehicleBrands,

    #[serde(rename = "1294")]
    VehicleCodesAndDrivingLaws,

    #[serde(rename = "1269")]
    VehicleFuelsAndLubricants,

    #[serde(rename = "170")]
    VehicleLicensingAndRegistration,

    #[serde(rename = "138")]
    VehicleMaintenance,

    #[serde(rename = "89")]
    VehiclePartsAndAccessories,

    #[serde(rename = "473")]
    VehicleShopping,

    #[serde(rename = "803")]
    VehicleShows,

    #[serde(rename = "1267")]
    VehicleSpecsReviewsAndComparisons,

    #[serde(rename = "438")]
    VehicleWheelsAndTires,

    #[serde(rename = "905")]
    VentureCapital,

    #[serde(rename = "793")]
    Veterans,

    #[serde(rename = "380")]
    Veterinarians,

    #[serde(rename = "1315")]
    VideoFileFormatsAndCodecs,

    #[serde(rename = "1342")]
    VideoGameEmulation,

    #[serde(rename = "1146")]
    VideoGameRetailers,

    #[serde(rename = "492")]
    VideoPlayersAndRecorders,

    #[serde(rename = "979")]
    VideoSharing,

    #[serde(rename = "1391")]
    VineyardsAndWineTourism,

    #[serde(rename = "972")]
    VirtualWorlds,

    #[serde(rename = "555")]
    VisaAndImmigration,

    #[serde(rename = "246")]
    VisionCare,

    #[serde(rename = "24")]
    VisualArtAndDesign,

    #[serde(rename = "237")]
    VitaminsAndSupplements,

    #[serde(rename = "618")]
    VocalsAndShowTunes,

    #[serde(rename = "369")]
    VocationalAndContinuingEducation,

    #[serde(rename = "386")]
    VoiceAndVideoChat,

    #[serde(rename = "865")]
    Volkswagen,

    #[serde(rename = "699")]
    Volleyball,

    #[serde(rename = "867")]
    Volvo,

    #[serde(rename = "451")]
    WarrantiesAndServiceContracts,

    #[serde(rename = "660")]
    WasteManagement,

    #[serde(rename = "987")]
    Watches,

    #[serde(rename = "1002")]
    WaterActivities,

    #[serde(rename = "441")]
    WaterAndMarineSciences,

    #[serde(rename = "1371")]
    WaterFiltersAndPurifiers,

    #[serde(rename = "118")]
    WaterSports,

    #[serde(rename = "1349")]
    WaterSupplyAndTreatment,

    #[serde(rename = "63")]
    Weather,

    #[serde(rename = "1142")]
    WebAppsAndOnlineTools,

    #[serde(rename = "422")]
    WebDesignAndDevelopment,

    #[serde(rename = "53")]
    WebHostingAndDomainRegistration,

    #[serde(rename = "301")]
    WebPortals,

    #[serde(rename = "302")]
    WebServices,

    #[serde(rename = "675")]
    WebStatsAndAnalytics,

    #[serde(rename = "575")]
    WebcamsAndVirtualTours,

    #[serde(rename = "293")]
    Weddings,

    #[serde(rename = "236")]
    WeightLoss,

    #[serde(rename = "706")]
    WelfareAndUnemployment,

    #[serde(rename = "683")]
    WesternEuropeans,

    #[serde(rename = "1099")]
    WesternFilms,

    #[serde(rename = "1225")]
    WholesalersAndLiquidators,

    #[serde(rename = "119")]
    Wildlife,

    #[serde(rename = "734")]
    WindowsAndDotNET,

    #[serde(rename = "737")]
    WindowsOS,

    #[serde(rename = "405")]
    Wine,

    #[serde(rename = "265")]
    WinterSports,

    #[serde(rename = "997")]
    WomenClothing,

    #[serde(rename = "648")]
    WomenHealth,

    #[serde(rename = "831")]
    WoodAndPlastics,

    #[serde(rename = "1345")]
    WordProcessingSoftware,

    #[serde(rename = "703")]
    WorkAndLaborIssues,

    #[serde(rename = "911")]
    WorldCuisines,

    #[serde(rename = "593")]
    WorldMusic,

    #[serde(rename = "1209")]
    WorldNews,

    #[serde(rename = "1198")]
    WorldSportsCompetitions,

    #[serde(rename = "512")]
    Wrestling,

    #[serde(rename = "1177")]
    WritersResources,

    #[serde(rename = "725")]
    WritingAndEditingServices,

    #[serde(rename = "1045")]
    Xbox,

    #[serde(rename = "953")]
    YardAndPatio,

    #[serde(rename = "611")]
    YogaAndPilates,

    #[serde(rename = "402")]
    YouthCamps,

    #[serde(rename = "1009")]
    ZoosAquariumsPreserves,
}
