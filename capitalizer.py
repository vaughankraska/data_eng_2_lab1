from pulsar import Function

 class RoutingFunction(Function):
     def __init__(self):
         self.capitalized = "persistent://public/default/upper"

     def process(self, input, context):
         context.publish(self.captialized, item.upper())
