#pragma once

#include "wrappers.hpp"
#include <QtGui/QCloseEvent>
#include <QtWidgets/QGridLayout>
#include <QtWidgets/QGroupBox>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  MainWindow(QWidget *parent = 0);
  virtual ~MainWindow(){ };
  void closeEvent(QCloseEvent *event);
private slots:
  void onClick();
  void loadPlugin();

private:
  PluginManager pm;
  QPushButton *btn_click;
  QPushButton *btn_plugin;
  QGroupBox *formGroupBox;
  QWidget *main_widget;
};
