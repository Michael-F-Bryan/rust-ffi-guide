#pragma once

#include "wrappers.hpp"
#include <QCloseEvent>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  MainWindow(QWidget *parent = 0);
  virtual ~MainWindow(){};
  void closeEvent(QCloseEvent *event);
private slots:
  void onClick();

private:
  QPushButton *button;
  PluginManager pm;
};
