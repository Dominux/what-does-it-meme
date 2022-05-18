# What Does It Meme

"What do you meme" table game implementation

## Game rules

### Starting the game

To start the game you need to create a room or enter some already created one by shared link to the room from the room creator or some other player that has the link. After entering a room you can share the link to the room to others you'd like to play with.

To start the game you need some minimum players limit to be entered the room. The minimum limit is not constant and may be changed by server administrator for all the rooms, but it's assumed to be not lesser than 4, cause in other case it may cause uninteresting game experience or even cause some bugs. Also there's a maximum players limit that's also is changeable by the administrator, but it is recomended to set it to a value around 10 for a fine playing experience!

After the minimum of players were added to the room you can click the "Start game" button to finally start the game (_such a surprisingly, unclear and absolutely unexpectable behavior_).

**10 minutes are alloted to start the game after room creation. Once this time is exceeded you are still able to start the game but if anyone will try to create another room and you won't have started the game - the room will be deleted as abandonded and you will need to create another and try again!**

### The game process

The game itself is sequential bunch of rounds that go one by one. Their amount is mutable too and by default it's set to 7.

Every round is made of some identical sequantial set of stages, where players should do some declared action. Every stage has a time to finish it and this time is represented by a countdown. **A next stage replace a current one once all the players that should do some stuff at this stage finish their actions or once the countdown become equal to zero.**

Further let's take a look at concrete stages:

##### Situation creation

Every round the game choose one player to be a situation creator. Situation creator should create a situation (_unexpectedly_) in this stage. Other players should just wait for him (or for the time to exceed). It's set 70 seconds by default for this stage.

##### Reacting with memes

Every player except of the situation creator should choose one meme from his collection to react to the situation. For this stage it's set 60 seconds by default.

##### Voting

Every player except of the situation creator should vote for one meme from the collection of choosed memes by other players. For this stage it's set 60 seconds by default.

##### Showing results

This stage does not require any actions from players. Here the players see the results of voting, they see authors and voters of every meme that was at this round. After that the players see total results for all the rounds by points they got. By default, we have 20 seconds to see the results.

### Calculating results

Every round every meme author gets equal amount of points for every voter of the meme they chosen. A situation creator gets a 1/2 part of the mean of the points values of top 2 memes authors of the round.
