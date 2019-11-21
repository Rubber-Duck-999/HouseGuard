package com;

import java.util.logging.Logger;
import com.house_guard.Common.*;

public class mainClass
{
    private static Model myModel;
    private static View myView;
    private static Controller myController;
    private static ConsumerTopic cons;

    public static void startUI()
    {
        int start_value = 0;
        myModel = new Model();
        myView = new View();

        MonitorView monitorView = new MonitorView();
        cons = new ConsumerTopic();
        myController = new Controller(myModel, myView, monitorView, cons, new RequestTable());
        myController.initmodel(start_value, Types.OFF);
        myView.addController(myController);
        monitorView.addController(myController);
        cons.consumeRequired();
    }

    public static void main(String[] argv) throws Exception
    {
        startUI();
    }
}
