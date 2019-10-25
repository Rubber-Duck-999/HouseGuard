package com;

import java.awt.event.ActionListener;
import java.awt.event.ActionEvent;

import java.util.logging.Logger;

public class Controller implements ActionListener
{
  public Model _model;
  public View _view;
  //private Logger _log;

  Controller(Model m, View v)//, Logger log)
  {
    //_log = log;
    //_log.info("Controller()");

    this._model = m;
    this._view = v;

    this._view.addCheckListener(new CheckListener());
  }

  public void actionPerformed(java.awt.event.ActionEvent e)
  {
    //_log.info()
    if(e.getActionCommand() == types.Actions.ADD.name())
    {
      _view.setNewValue(_model.incrementValue());
    }
    else if(e.getActionCommand() == types.Actions.MINUS.name())
    {
      _view.setNewValue(_model.decrementValue());
    }
  }

  class CheckListener implements ActionListener
  {
    @Override
    public void actionPerformed(ActionEvent e)
    {
      //_log.info("Controller: acting");
      if(_model.createFile())
      {
        _view.setNewFrame(_model.createFile());
      }
      else
      {
        _view.displayErrorMessage(_model.getMessageReceived());
      }
    }
  }

  public void initmodel(int x)
  {
    _view.setNewValue(_model.setModelValue(x));
  }
}
