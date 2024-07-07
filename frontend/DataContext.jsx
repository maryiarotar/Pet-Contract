import React, { useState } from 'react';

const AddProjectForm = () => {
  const [formData, setFormData] = useState({
    id: 1, 
    name: '',
    topic: '',
    file_path: '',
    description: '',
    author: '',
    date_published: ''
  });

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData(prevState => ({
      ...prevState,
      [name]: value
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
        console.log("params: " + formData.name + " " + formData.author );
      const response = await fetch('http://127.0.0.1:3000/add', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(formData)
      });
      if (response.ok) {
        console.log('Project added successfully!');
        // Дополнительные действия после успешной отправки
      } else {
        console.error('Failed to add project.');
      }
    } catch (error) {
      console.error('Error adding project:', error);
    }
  };

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '100vh', background: 'indigo', color: 'white' }}>
      <form onSubmit={handleSubmit} style={{ width: '400px', background: 'black', padding: '20px', borderRadius: '8px', boxShadow: '0px 0px 10px 0px rgba(0,0,0,0.5)' }}>
        <h2 style={{ textAlign: 'center', color: 'indigo', marginBottom: '20px' }}>Add New Project</h2>
        <label>
          Name:
          <input type="text" name="name" value={formData.name} onChange={handleChange} style={{ width: '100%', padding: '8px', marginBottom: '10px' }} />
        </label>
        <label>
          Topic:
          <textarea name="topic" value={formData.topic} onChange={handleChange} style={{ width: '100%', padding: '8px', marginBottom: '10px' }} />
        </label>
        <label>
          Description:
          <textarea name="description" value={formData.description} onChange={handleChange} style={{ width: '100%', padding: '8px', marginBottom: '10px' }} />
        </label>
        <label>
          File Path:
          <input type="text" name="file_path" value={formData.file_path} onChange={handleChange} style={{ width: '100%', padding: '8px', marginBottom: '10px' }} />
        </label>
        <label>
          Date Published:
          <input type="text" name="date_published" value={formData.date_published} onChange={handleChange} style={{ width: '100%', padding: '8px', marginBottom: '10px' }} />
        </label>
        <label>
          Author:
          <input type="text" name="author" value={formData.author} onChange={handleChange} style={{ width: '100%', padding: '8px', marginBottom: '10px' }} />
        </label>
        <button type="submit" style={{ width: '100%', padding: '10px', background: 'indigo', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>Submit</button>
      </form>
    </div>
  );
};

export default AddProjectForm;
