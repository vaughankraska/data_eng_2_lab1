from pulsar import Function

 class RoutingFunction(Function):
     def __init__(self):
         self.split_words = "persistent://public/default/split"

     def process(self, input, context):
         for word in input.split(" "):
             context.publish(self.split_words, word);
