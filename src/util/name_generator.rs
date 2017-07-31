extern crate rand;


use components::living::Name;


const FEMALE_NAMES: &'static [&str] = &[
    "Lily", "Phyllis", "Agnes", "Adrienne",
    "Jennifer", "Amy", "Melissa", "Michelle",
    "Kimberly", "Lisa", "Angela", "Heather",
    "Stephanie", "Nicole", "Jessica", "Elizabeth",
    "Rebecca", "Kelly", "Mary", "Christina",
    "Amanda", "Julie", "Sarah", "Laura",
    "Shannon", "Christine", "Tammy", "Tracy",
    "Karen", "Dawn", "Susan", "Andrea",
    "Tina", "Patricia", "Cynthia", "Lori",
    "Rachel", "April", "Maria", "Wendy",
    "Crystal", "Stacy", "Erin", "Jamie",
    "Carrie", "Tiffany", "Tara", "Sandra",
    "Monica", "Danielle", "Stacey", "Pamela",
    "Tonya", "Sara", "Michele", "Teresa",
    "Denise", "Jill", "Katherine", "Melanie",
    "Dana", "Holly", "Erica", "Brenda",
    "Deborah", "Tanya", "Sharon", "Donna",
    "Amber", "Emily", "Robin", "Linda",
    "Kathleen", "Leslie", "Christy", "Kristen",
    "Catherine", "Kristin", "Misty", "Barbara",
    "Heidi", "Nancy", "Cheryl", "Theresa",
    "Brandy", "Alicia", "Veronica", "Gina",
    "Jacqueline", "Rhonda", "Anna", "Renee",
    "Megan", "Tamara", "Melinda", "Kathryn",
    "Debra", "Sherry", "Allison", "Valerie",
    "Diana", "Paula", "Kristina", "Ann",
    "Margaret", "Cindy", "Victoria", "Jodi",
    "Natalie", "Brandi", "Suzanne", "Kristi",
    "Samantha", "Beth", "Tracey", "Regina",
    "Vanessa", "Kristy", "Carolyn", "Yolanda",
    "Deanna", "Carla", "Sheila", "Laurie",
    "Anne", "Shelly", "Diane", "Sabrina",
    "Janet", "Erika", "Katrina", "Courtney",
    "Colleen", "Carol", "Julia", "Jenny",
    "Jaime", "Kathy", "Felicia", "Alison",
    "Lauren", "Kelli", "Leah", "Ashley",
    "Kim", "Traci", "Kristine", "Tricia",
    "Joy", "Krista", "Kara", "Terri",
    "Sonya", "Aimee", "Natasha", "Cassandra",
    "Bridget", "Anita", "Kari", "Nichole",
    "Christie", "Marie", "Virginia", "Connie",
    "Martha", "Carmen", "Stacie", "Lynn",
    "Katie", "Monique", "Kristie", "Shelley",
    "Sherri", "Angel", "Bonnie", "Mandy",
    "Jody", "Shawna", "Kerry", "Annette",
    "Yvonne", "Toni", "Meredith", "Molly",
    "Kendra", "Joanna", "Sonia", "Janice",
    "Robyn", "Brooke", "Kerri", "Sheri",
    "Becky", "Gloria", "Mindy", "Tracie",
    "Angie", "Kellie", "Claudia", "Ruth",
    "Wanda", "Jeanette", "Cathy", "Cecilia",
    "Clarice", "Hope", "Ronda", "Daisy",
    "Pam",
];

const MALE_NAMES: &'static [&str] = &[
    "Bob", "Ted", "Cletus", "Michael",
    "Christopher", "Jason", "David", "James",
    "John", "Robert", "Brian", "William",
    "Matthew", "Joseph", "Daniel", "Kevin",
    "Eric", "Jeffrey", "Richard", "Scott",
    "Mark", "Steven", "Thomas", "Timothy",
    "Anthony", "Charles", "Joshua", "Ryan",
    "Jeremy", "Paul", "Andrew", "Gregory",
    "Chad", "Kenneth", "Jonathan", "Stephen",
    "Shawn", "Aaron", "Adam", "Patrick",
    "Justin", "Sean", "Edward", "Todd",
    "Donald", "Ronald", "Benjamin", "Keith",
    "Bryan", "Gary", "Jose", "Nathan",
    "Douglas", "Brandon", "Nicholas", "George",
    "Travis", "Peter", "Craig", "Bradley",
    "Larry", "Dennis", "Shane", "Raymond",
    "Troy", "Jerry", "Samuel", "Frank",
    "Jesse", "Jeffery", "Juan", "Terry",
    "Corey", "Phillip", "Marcus", "Derek",
    "Rodney", "Joel", "Carlos", "Randy",
    "Jacob", "Jamie", "Tony", "Russell",
    "Brent", "Antonio", "Billy", "Derrick",
    "Kyle", "Erik", "Johnny", "Marc",
    "Carl", "Philip", "Roger", "Bobby",
    "Brett", "Danny", "Curtis", "Jon",
    "Vincent", "Cory", "Jimmy", "Victor",
    "Lawrence", "Dustin", "Gerald", "Walter",
    "Alexander", "Joe", "Christian", "Chris",
    "Alan", "Shannon", "Wayne", "Jared",
    "Gabriel", "Martin", "Jay", "Luis",
    "Willie", "Micheal", "Henry", "Wesley",
    "Randall", "Brad", "Darren", "Roy",
    "Albert", "Arthur", "Ricky", "Lance",
    "Allen", "Lee", "Bruce", "Andre",
    "Mario", "Frederick", "Louis", "Darrell",
    "Damon", "Shaun", "Nathaniel", "Zachary",
    "Casey", "Adrian", "Jesus", "Jeremiah",
    "Jack", "Ronnie", "Dale", "Tyrone",
    "Manuel", "Ricardo", "Harold", "Kelly",
    "Barry", "Ian", "Reginald", "Glenn",
    "Ernest", "Steve", "Seth", "Eugene",
    "Clinton", "Miguel", "Tommy", "Eddie",
    "Leonard", "Maurice", "Roberto", "Dwayne",
    "Jerome", "Ralph", "Marvin", "Francisco",
    "Jorge", "Neil", "Alex", "Dean",
    "Kristopher", "Calvin", "Kurt", "Theodore",
    "Ruben", "Jermaine", "Tracy", "Edwin",
    "Stanley", "Melvin", "Howard", "Mitchell",
    "Duane", "Trevor", "Jeff", "Geoffrey",
    "Hector", "Terrence", "Terrance", "Oscar",
    "Jaime", "Clifford", "Harry", "Sven",
    "Herbert", "Bender", "Clark", "Parker",
    "Bart", "Bryce", "Simon", "Art",
    "Jim", "Wallace", "Rick",
];


const LAST_NAMES: &'static  [&str] = &[
    "Smith", "Johnson", "Williams", "Jones",
    "Brown", "Davis", "Miller", "Wilson",
    "Moore", "Taylor", "Anderson", "Thomas",
    "Jackson", "White", "Harris", "Martin",
    "Thompson", "Garcia", "Martinez", "Robinson",
    "Clarke", "Rodriguez", "Lewis", "Lee",
    "Walker", "Hall", "Allen", "Young",
    "Hernandez", "King", "Haase", "Wick",
    "Woodhouse", "Wick", "Svensson", "Herbert",
    "Sherman", "Herrman", "Burton", "Simpson",
    "Fry", "Farnsworth", "Baggins", "Bronson",
    "Branson", "Kent", "Wayne", "Stark",
    "Parker", "Creutzfeldt", "Jacobs", "Berg",
    "Kerbin", "Rice", "Simon", "Garfunkel",
    "Starling", "Rousey", "Vance", "Dunder",
    "Mifflin", "California", "Wallace", "Wong",
    "Cheese", "Sanchez",
];


pub struct NameGenerator;

impl NameGenerator {
    pub fn gen_female() -> String {
        let idx = rand::random::<usize>() % FEMALE_NAMES.len();

        String::from(FEMALE_NAMES[idx])
    }

    pub fn gen_male() -> String {
        let idx = rand::random::<usize>() % MALE_NAMES.len();

        String::from(MALE_NAMES[idx])
    }

    pub fn gen_last() -> String {
        let idx = rand::random::<usize>() % LAST_NAMES.len();

        String::from(LAST_NAMES[idx])
    }

    pub fn gen_name() -> Name {
        if rand::random::<bool>() {
            Name {
                firstname: Self::gen_female(),
                lastname: Self::gen_last(),
            }
        } else {
            Name {
                firstname: Self::gen_male(),
                lastname: Self::gen_last(),
            }
        }
    }
}
