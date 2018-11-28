// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

test('test handle_log', (t) => {
  // indicates the number of assertions that follow
  // t.plan(1)

  // Make a call to a Zome function
  // indicating the group and function, and passing it an input
  // const result = app.call("zome-name", "capability-name", "function-name", {})
  const result = app.call("service", "main", "logger", {stuff:"test"} )

  // check for equality of the actual and expected results
  // t.equal(result, "expected result!")
//  t.equal(result, "blah", "Error calling zome function: InternalFailure(Dna(ZomeNotFound(\"Zome \\\'zome-name\\\' not found\")))")
  t.deepEqual(result.rawResult, 'test' )

  // ends this test
  t.end()
})
