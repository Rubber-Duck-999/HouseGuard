package com;

import java.awt.BorderLayout;
import java.awt.FlowLayout;
import java.awt.GridBagConstraints;
import java.awt.GridBagLayout;
import java.awt.event.*;

import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JOptionPane;
import javax.swing.JPanel;
import javax.swing.JTextField;

import java.util.logging.Logger;

public class View
{
  private JFrame frame;
  private JPanel buttonPanel, fieldPanel;
  private JLabel counter, check;
  private JTextField textField, inputField;
  private JButton addButton, minusButton, checkFileButton;

  View()//(Logger log)
  {
    //log.finest("Starting View");
    frame = new JFrame("User Panel");
    buttonPanel = new JPanel();
    fieldPanel = new JPanel();

    counter = new JLabel("Counter");
    check = new JLabel("Check File");

    textField = new JTextField("");

    addButton = new JButton("Add");
    minusButton = new JButton("Sub");
    checkFileButton = new JButton("Check if exists");

    fieldPanel.setLayout(new GridBagLayout());
    GridBagConstraints gbc = new GridBagConstraints();

    gbc.fill = GridBagConstraints.HORIZONTAL;
    gbc.gridx = 0;
    gbc.gridy = 0;
    fieldPanel.add(counter, gbc);

    gbc.gridx = 0;
    gbc.gridy = 1;
    fieldPanel.add(textField, gbc);

    gbc.gridx = 1;
    gbc.gridy = 0;
    fieldPanel.add(check, gbc);

    buttonPanel.setLayout(new FlowLayout());
    buttonPanel.add(addButton);
    buttonPanel.add(minusButton);
    buttonPanel.add(checkFileButton);

    frame.add(fieldPanel, BorderLayout.PAGE_START);
    frame.add(buttonPanel, BorderLayout.PAGE_END);

    frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
    frame.setSize(640,480);
    frame.pack();
    frame.setVisible(true);
  }

  public void setNewFrame(boolean visible)
  {
    //log.info("Set New Frame = " + visible);
  }

  public < E > void setNewValue(E v)
  {
    //log.info("Set Value");
    textField.setText("" + v);
  }

  public void addCheckListener(ActionListener listenForCheckButton)
  {
    //log.info("Adding Check Listener");
    checkFileButton.setActionCommand(types.Actions.CHECK.name());
    checkFileButton.addActionListener(listenForCheckButton);
  }

  public void addController(ActionListener listenerButtons)
  {
    //log.info("Adding Controller to View");
    addButton.setActionCommand(types.Actions.ADD.name());
    minusButton.setActionCommand(types.Actions.MINUS.name());
    addButton.addActionListener(listenerButtons);
    minusButton.addActionListener(listenerButtons);
  }

  public void displayErrorMessage(String message)
  {
    JOptionPane.showMessageDialog(frame, "Error: " + message, "Error", JOptionPane.ERROR_MESSAGE);
  }

  public static class CloseListener extends WindowAdapter
  {
    public void windowClosing(WindowEvent e)
    {
      e.getWindow().setVisible(false);
      System.exit(0);
    }
  }


}
