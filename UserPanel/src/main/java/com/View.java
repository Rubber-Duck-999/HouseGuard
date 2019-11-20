package com;

import java.awt.BorderLayout;
import com.house_guard.Common.*;
import java.awt.FlowLayout;
import java.awt.Font;
import java.awt.GraphicsDevice;
import java.awt.GridBagConstraints;
import java.awt.GridBagLayout;
import java.awt.GridLayout;
import java.awt.event.*;

import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JOptionPane;
import javax.swing.JPanel;
import javax.swing.JTextField;
import javax.swing.SwingConstants;

public class View
{
	private JFrame frame;
	private JPanel buttonPanel, fieldPanel;
	private JLabel passFieldDigit1, passFieldDigit2, passFieldDigit3, passFieldDigit4;
	private JButton addButtonD1, subButtonD1;
	private JButton addButtonD2, subButtonD2;
	private JButton addButtonD3, subButtonD3;
	private JButton addButtonD4, subButtonD4;
	private JButton enter;
	private Font buttonFont;

	public View()
	{
		frame = new JFrame("Simple MVC");
		buttonPanel = new JPanel();
		fieldPanel = new JPanel();

		passFieldDigit1 = new JLabel("", SwingConstants.CENTER);
		passFieldDigit2 = new JLabel("", SwingConstants.CENTER);
		passFieldDigit3 = new JLabel("", SwingConstants.CENTER);
		passFieldDigit4 = new JLabel("", SwingConstants.CENTER);

		addButtonD1 = new JButton("+");
		subButtonD1 = new JButton("-");

		addButtonD2 = new JButton("+");
		subButtonD2 = new JButton("-");

		addButtonD3 = new JButton("+");
		subButtonD3 = new JButton("-");

		addButtonD4 = new JButton("+");
		subButtonD4 = new JButton("-");

		buttonPanel.add(addButtonD1);
		buttonPanel.add(addButtonD2);
		buttonPanel.add(addButtonD3);
		buttonPanel.add(addButtonD4);
		buttonPanel.add(subButtonD1);
		buttonPanel.add(subButtonD2);
		buttonPanel.add(subButtonD3);
		buttonPanel.add(subButtonD4);

		enter = new JButton("Enter");
		enter.setFont(new Font("Calibri", Font.BOLD, 28));
		buttonPanel.setLayout(new GridLayout(2,4));

		setButtonFont();
		setLabelFont();

		fieldPanel.setLayout(new GridLayout(1,2));
		fieldPanel.add(passFieldDigit1);
		fieldPanel.add(passFieldDigit2);
		fieldPanel.add(passFieldDigit3);
		fieldPanel.add(passFieldDigit4);

		frame.add(fieldPanel, BorderLayout.NORTH);
		frame.add(buttonPanel, BorderLayout.CENTER);
		frame.add(enter, BorderLayout.SOUTH);

		GraphicsDevice device = frame.getGraphicsConfiguration().getDevice();
		
		frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		frame.setExtendedState(JFrame.MAXIMIZED_BOTH);
		//frame.setUndecorated(true);
		frame.setSize(1280,980);
		//frame.pack();
		frame.setVisible(true);
		device.setFullScreenWindow(frame);
	}

	private void setLabelFont()
	{
		buttonFont = new Font("Calibri", Font.BOLD, 128);
		passFieldDigit1.setFont(buttonFont);
		passFieldDigit2.setFont(buttonFont);
		passFieldDigit3.setFont(buttonFont);
		passFieldDigit4.setFont(buttonFont);
	}

	private void setButtonFont()
	{
		buttonFont = new Font("Calibri", Font.BOLD, 40);
		addButtonD1.setFont(buttonFont);
		subButtonD1.setFont(buttonFont);

		addButtonD2.setFont(buttonFont);
		subButtonD2.setFont(buttonFont);

		addButtonD3.setFont(buttonFont);
		subButtonD3.setFont(buttonFont);

		addButtonD4.setFont(buttonFont);
		subButtonD4.setFont(buttonFont);
	}

	public < E > void setDigit1(E v)
	{
		System.out.println(v);
		passFieldDigit1.setText("" + v);
	}

	public < E > void setDigit2(E v)
	{
		passFieldDigit2.setText("" + v);
	}

	public < E > void setDigit3(E v)
	{
		passFieldDigit3.setText("" + v);
	}

	public < E > void setDigit4(E v)
	{
		passFieldDigit4.setText("" + v);
	}

	public < E > void setDigits(E v)
	{
		passFieldDigit1.setText("" + v);
		passFieldDigit2.setText("" + v);
		passFieldDigit3.setText("" + v);
		passFieldDigit4.setText("" + v);
	}

	public void addController(ActionListener listenerButtons)
	{
		addButtonD1.setActionCommand(Types.Actions.ADD_D1.name());
		subButtonD1.setActionCommand(Types.Actions.SUB_D1.name());
		addButtonD1.addActionListener(listenerButtons);
		subButtonD1.addActionListener(listenerButtons);

		addButtonD2.setActionCommand(Types.Actions.ADD_D2.name());
		subButtonD2.setActionCommand(Types.Actions.SUB_D2.name());
		addButtonD2.addActionListener(listenerButtons);
		subButtonD2.addActionListener(listenerButtons);

		addButtonD3.setActionCommand(Types.Actions.ADD_D3.name());
		subButtonD3.setActionCommand(Types.Actions.SUB_D3.name());
		addButtonD3.addActionListener(listenerButtons);
		subButtonD3.addActionListener(listenerButtons);

		addButtonD4.setActionCommand(Types.Actions.ADD_D4.name());
		subButtonD4.setActionCommand(Types.Actions.SUB_D4.name());
		addButtonD4.addActionListener(listenerButtons);
		subButtonD4.addActionListener(listenerButtons);

		enter.setActionCommand(Types.Actions.ENTER.name());
		enter.addActionListener(listenerButtons);
	}

	public void displayErrorMessage(String message)
	{
		JOptionPane.showMessageDialog(frame, "Error: " + message, "Error", JOptionPane.ERROR_MESSAGE);
	}

	public void displayPassMessage(String message)
	{
		JOptionPane.showMessageDialog(frame, "Pass: " + message, "Pass", JOptionPane.INFORMATION_MESSAGE);
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
