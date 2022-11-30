from flask_wtf import FlaskForm
from wtforms import StringField, BooleanField, SubmitField, FloatField, IntegerField, SelectField
from wtforms.validators import DataRequired, NumberRange, Length

class GetStartedForm(FlaskForm):
    submit = SubmitField("Configure New Measurement")

class SetupMeasurementForm(FlaskForm):
    measFreq = FloatField('Frequency (Hz)', validators=[DataRequired(), NumberRange(max=1000)])
    duration = IntegerField('Duration (Seconds)', validators=[DataRequired(), NumberRange(min=0)])
    numAvgs = IntegerField('Number of averaged measurements', validators=[DataRequired(), NumberRange(min=1)])
    measName = StringField('Measurement name', validators=[DataRequired(), Length(max=50)])
    #resBW = SelectField('Resolution Bandwidth', choices=[('high sense', 'High Sensitivity'), ('medium', 'Medium Sensitivity'), ('speed', 'High Speed')])
    #preampEnabled = BooleanField('Preamp Enabled')
    submit = SubmitField('Configure Hotpi Measurement')

class MissionStartForm(FlaskForm):
    request = SubmitField('Start Recording')

class MissionResultsForm(FlaskForm):
    request = SubmitField('Stop Recording')
