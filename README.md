[![Build Status](https://dev.azure.com/simoncrowther95/simoncrowther95/_apis/build/status/Rubber-Duck-999.HouseGuard?branchName=master)](https://dev.azure.com/simoncrowther95/simoncrowther95/_build/latest?definitionId=6&branchName=master)


HouseGuard

![GitHub Logo](/images/archiecture.png)
Format: ![Alt Text](url)

A project based on standalone on user controlled security system that can manage multi sensor inputs and handle external communication. Complete with its own network security and message broker design. 

Uses rabbitmq with the amqp protocol to communicate between software components that have been partitiioned into functional boxes that have defined operations.

Can monitor and detect movement on a raspi camera
Contact relevant authorities if movement is severe and breached the property

Full design is attached in a tagged branch of the repository
