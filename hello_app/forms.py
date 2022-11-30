from flask_wtf import FlaskForm
from wtforms import StringField, BooleanField, SubmitField, FloatField, IntegerField, SelectField
from wtforms.validators import DataRequired, NumberRange

class GetStartedForm(FlaskForm):
    submit = SubmitField("Configure New Measurement")

class SetupMeasurementForm(FlaskForm):
    startFreq = FloatField('Frequency (Hz)', validators=[DataRequired(), NumberRange(max=1000)])
    duration = FloatField('Duration (Seconds)', validators=[DataRequired(), NumberRange(min=0)])
    numAvgs = FloatField('Number of averaged measurements', validators=[DataRequired(), NumberRange(min=1)])
    stopFreq = FloatField('Stop Frequency (MHz)', validators=[DataRequired(), NumberRange(min=0.03,max=50000)])
    intBW = FloatField('Integration Bandwidth (MHz)', validators=[DataRequired(), NumberRange(min=0.001)])
    resBW = SelectField('Resolution Bandwidth', choices=[('high sense', 'High Sensitivity'), ('medium', 'Medium Sensitivity'), ('speed', 'High Speed')])
    preampEnabled = BooleanField('Preamp Enabled')
    rfAtten = SelectField('RF Attenuation', choices=[(0, '0dB'), (5, '5dB'), (10, '10dB'), (15, '15dB'), (20, '20dB'), (25, '25dB'), (30, '30dB')])
    interval = IntegerField('Time Between Measurements (seconds)', validators=[DataRequired(), NumberRange(min=1)])
    submit = SubmitField('Configure FieldFox')

class MissionStartForm(FlaskForm):
    request = SubmitField('Start Recording')

class MissionResultsForm(FlaskForm):
    request = SubmitField('Stop Recording')


