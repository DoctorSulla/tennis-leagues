// Function to get the leagues
async function getLeagues() {
  let request = await fetch("/api/leagues");
  let response = await request.json();
  return response;
}

// Function to get the fixtures and table for a specific league
async function getLeague(league) {
  document.querySelector("#league-name").innerHTML = league.league_name;

  document.querySelector("#league-logo").src =
    "/img/" + league.league_id + ".png";

  let request = await fetch("/api/leagueTable/" + league.league_id);
  let response = await request.json();
  return response;
}

// Function to parse the response
function populateResponse(league) {
  reset();
  let position = 1;
  for (let row of league.league_table) {
    let tr = document.createElement("tr");
    tr.innerHTML = `
          <td><div class='league-position'>${position}</div></td>
          <td >${row.name}</td>
          <td>${row.played}</td>
          <td>${row.matches_won}</td>
          <td>${row.matches_lost}</td>
          <td>${row.sets_won}</td>
          <td>${row.sets_lost}</td>
          <td class='mobile-hidden'>${row.games_won}</td>
          <td class='mobile-hidden'>${row.games_lost}</td>
          <td><b>${row.points}</b></td>
          `;
    document.querySelector("#league-table-body").append(tr);
    position++;
  }

  let completedFixturesDiv = document.querySelector("#completed-fixtures");
  if (league.completed_fixtures.length == 0) {
    completedFixturesDiv.innerHTML = "<p>No completed fixtures yet.</p>";
  } else {
    completedFixturesDiv.innerHTML = "";
  }

  for (let fixture of league.completed_fixtures) {
    let div = document.createElement("div");
    div.classList.add("completed-fixture");
    div.innerHTML = `<table>
          <tr class='result-header'><th></th><th>1</th><th>2</th><th>3</th></tr>
          <tr><td ${fixture.player_one_id == fixture.winner ? "style='font-weight:bold'" : ""}>${fixture.player_one_name}</td><td>${fixture.player_one_set_one_games}</td><td>${fixture.player_one_set_two_games}</td>
          <td>${fixture.player_one_tiebreak_points ? fixture.player_one_tiebreak_points : " - "}</td>
          </tr>
          <tr><td ${fixture.player_two_id == fixture.winner ? "style='font-weight:bold'" : ""}>${fixture.player_two_name}</td><td>${fixture.player_two_set_one_games}</td><td>${fixture.player_two_set_two_games}</td>
          <td>${fixture.player_two_tiebreak_points ? fixture.player_two_tiebreak_points : " - "}</td>
          </tr>
          </table>
          `;
    completedFixturesDiv.append(div);
  }

  let uncompletedFixturesDiv = document.querySelector("#uncompleted-fixtures");
  if (league.uncompleted_fixtures.length == 0) {
    uncompletedFixturesDiv.innerHTML = "<p>All fixtures are complete.</p>";
  } else {
    uncompletedFixturesDiv.innerHTML = "";
  }

  for (let fixture of league.uncompleted_fixtures) {
    let div = document.createElement("div");
    div.classList.add("uncompleted-fixture");
    div.innerHTML = `${fixture.player_one_name} vs ${fixture.player_two_name}`;
    uncompletedFixturesDiv.append(div);
  }
}

// Function to reset the divs
function reset() {
  document.querySelector("#league-table-body").innerHTML = "";
  document.querySelector("#completed-fixtures").innerHTML = "";
  document.querySelector("#uncompleted-fixtures").innerHTML = "";
}

let leagueIndex = 0;
let leagues = [];

// Initial fetch + event listeners
document.addEventListener("DOMContentLoaded", async function () {
  leagues = await getLeagues();
  let league = await getLeague(leagues[leagueIndex]);
  populateResponse(league);

  document
    .querySelector("#left-nav")
    .addEventListener("click", async function () {
      if (leagueIndex == 0) {
        leagueIndex = leagues.length - 1;
      } else {
        leagueIndex--;
      }
      const league = await getLeague(leagues[leagueIndex]);
      populateResponse(league);
    });

  document
    .querySelector("#right-nav")
    .addEventListener("click", async function () {
      if (leagueIndex == leagues.length - 1) {
        leagueIndex = 0;
      } else {
        leagueIndex++;
      }
      const league = await getLeague(leagues[leagueIndex]);
      populateResponse(league);
    });

  document
    .querySelector("#league-table")
    .addEventListener("animationend", function () {
      console.log("The animation is finished");
    });
});
