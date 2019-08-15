import provingKey from './provingkey.txt';

var startTime, endTime;

function start() {
  startTime = new Date();
  console.log("starting...")
};

function end() {
  endTime = new Date();
  var timeDiff = endTime - startTime; //in ms
  // strip the ms
  timeDiff /= 1000;

  // get seconds 
  var seconds = Math.round(timeDiff);
  console.log(seconds + " seconds");
}

async function go() {
    const rust = await import('./pkg/bellwasm');

    start();
    rust.test_speed();
    end();


    start()
    console.log(rust.proof(provingKey))
    end()
} 

go()
