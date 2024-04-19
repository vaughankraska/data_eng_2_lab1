# Data Engineering II Lab I
## My implementation
- Use rust to implement the producers and consumers
- Use the gutenberg project disribution of the pride and prejudice book as the text we are are performing our capization process on. Link: https://www.gutenberg.org/ebooks/1342.txt.utf-8
    - (mini example included in repo as pride_mini.txt)
- Use pulsar functions to complete the functionality of the below python function (splitting/capitalization)

- Notes on repo: 
    * the docker stuff is broken and not working (using docker compose does not work for deploying locally or on any server)
    * You have to start the pulsar functions on every restart or attach the pulsar Function config files
    * You have to upload the pulsar function files to the pulsar cluster
    * exec into docker container as root with: docker exec -u root -it <containerid> /bin/bash

## To run
* start pulsar (with docker) in a session:
```bash
docker run -it \
-p 6650:6650 \
-p 8080:8080 \
--mount source=pulsardata,target=/pulsar/data \
--mount source=pulsarconf,target=/pulsar/conf \
apachepulsar/pulsar:3.2.2 \
bin/pulsar standalone
```
* upload pulsar functions to pulsar instance:
```bash
docker cp functions/<function_file>.py <CONTAINER_ID>:/pulsar
```
* upload/replace pulsar config:
```bash
docker cp standalone.conf <CONTAINER_ID>:/pulsar/conf/standalone.conf
```
OR just add functionsWorkerEnabled=true to existing standalone.conf
* Start Capitalizer
```bash
bin/pulsar-admin functions localrun   --py $PWD/capitalizer.py   --classname capitalizer   --inputs persistent://public/default/split   --output persistent://public/default/upper
```

* Start Splitter
```bash
bin/pulsar-admin functions localrun   --py $PWD/splitter.py   --classname splitter.SplitterFunc   --inputs persistent://public/default/raw   --output persistent://public/default/split
```
OR
```bash
bin/pulsar-admin functions create \
  --name cap
  --py $PWD/capitalizer.py \
  --classname capitalizer \
  --inputs persistent://public/default/split
  --output persistent://public/default/upper

bin/pulsar-admin functions create \
  --name split
  --py $PWD/splitter.py \
  --classname splitter.SplitterFunc \
  --inputs persistent://public/default/raw
  --output persistent://public/default/split
```
and start them:
```bash
bin/pulsar-admin functions start \
    --tenant public \
    --namespace default \
    --name cap \
bin/pulsar-admin functions start \
    --tenant public \
    --namespace default \
    --name split \
```

* start consumer: cd into /consumer
```bash
RUST_LOG=info cargo run --release
```
* run producer (with full text file): cd into /producer
```bash
RUST_LOG=info FILE_PATH=pride_and_pred.txt cargo run --release
```


## Messaging flow:
text file -> 
    producer -> 
        'raw' (topic) -> 
            splitter (Function) -> 
                'split' (topic) -> 
                    capitalizer (Function) ->
                        'upper' (topic) ->
                            consumer ->
                                merged to 'output.txt' file
## Assignment Description:
```text
The repository consists of a conversion.py file which demonstrates a code that has a
“conversion” operation applied to each word on a string. Assume that there is a requirement that
the operation is applied on each word instead of the entire string. Based on this premise, you
are required to complete the following sub-tasks:
1. Identify an issue with the current implementation in terms of handling big data i.e., words
are in the order of millions.
2. Redesign the current implementation by using apache pulsar to demonstrate splitting
and merging of data i.e., how the same operation on each word (i.e., splitting) and the
resultant string (i.e., merging) can be handled by consumers and producers. Provide a
diagram to demonstrate how your architecture will look like i.e.,
a. How data splitting and merging is handled by consumer and producer
b. Label broker, consumer and producer.
3. Provide an implementation (preferably in Python programming language) of your
architecture with Apache Pulsar. You can set up your architecture on a single virtual
machine by creating different sessions for each consumer and producer.
```

* The file:

```python
#!/usr/bin/env python3
import time

"""
    The implementation is used to demonstrate an intensive "conversion" function on elements
"""

# Fill in your author information
___author___ = ""
___email____ = ""

# Input string
INPUT_STRING = "I want to be capatilized"

# Iteration represents the operation applied to each word in the string 
# e.g., 1 represents that operation is applied to first word in the string
ITERATION = 5

def conversion(substring, operation):
    """A conversion function which takes a string as an input and outputs a converted string

    Args:
        substring (String)
        operation (function): This is an operation on the given input

    Returns:
        [String]: Converted String
    """


    # returns the conversion applied to input
    return function(substring)



def function(string):
    """ A function that performs some operation on a string. You can change the operation accordingly

    Args:
        string (String): input string on which some operation is applied

    Returns:
        [String]: string in upper case
    """
    return string.upper()





if __name__ == "__main__":

    # Check for correct input
    if ITERATION > len(INPUT_STRING.split()):
        print ("Iteration cannot be greater than the number of words in a string")
        print ("Terminating the benchmark")
        exit()

    print ("Original String: {}".format(INPUT_STRING))
    resultant_string = ""
    


    split_string = INPUT_STRING.split(" ")


    for i in range(0,ITERATION):

        upper_case_string = conversion(split_string[i], function)
        
        resultant_string +=   upper_case_string  + ' '    


    print ("Resultant String: {}".format(resultant_string))
```
